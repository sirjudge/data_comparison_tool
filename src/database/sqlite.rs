use crate::{
    models::{
        comparison_data::ComparisonData,
        table_data::TableData,
    },
    log::Log
};
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::SqlitePoolOptions,
    SqlitePool,
    Column,
    Pool,
};

/// open a connection to the sqlite database
pub(crate) async fn get_sqlite_connection(log: &Log) -> Pool<sqlx::Sqlite> {
    let db_url = "sqlite://./db.sqlite3";
    // check if sqlite database exists and create it if it doesn't
    if !sqlx::Sqlite::database_exists(db_url).await.unwrap() {
        sqlx::Sqlite::create_database(db_url).await.unwrap();
        log.info("database did not previously exist, created sqlite db");
    }

    // connect to the sqlite database and return the pool
    let result = SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(db_url)
        .await;
    match result {
        Ok(pool) => pool,
        Err(error) => {
            panic!("unable to connect to sqlite db {}", error);
        },
    }
}

/// Compare two sqlite tables and return the differences
pub(crate) async fn compare_sqlite_tables(
    table_data_1: &TableData,
    table_data_2: &TableData,
    mut create_sqlite_comparison_files: bool,
    in_memory_sqlite: bool,
    log: &Log,
    auto_yes: bool
) -> ComparisonData {
    if in_memory_sqlite && create_sqlite_comparison_files {
        log.info("using in memory sqlite for data comparison,
             this will be faster but will not save the comparison
             data to disk, do you want to continue? (yes/no)",
        );

        // read from std in
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        //TODO: take in input args auto-yes flag to pass down to here
        if input == "yes\n" || input == "y" || auto_yes {
            log.info("continuing with in memory sqlite");
            create_sqlite_comparison_files = false;
        } else {
            log.info("exiting program");
            std::process::exit(0);
        }
    }

    // get the sqlite connection, and execute each part of the comparison
    let sqlite_pool = self::get_sqlite_connection(log).await;

    let comparison_data = ComparisonData::new(
        get_unique_rows(
            table_data_1,
            table_data_2,
            &sqlite_pool,
            create_sqlite_comparison_files,
            log,
        )
        .await,
        get_unique_rows(
            table_data_2,
            table_data_1,
            &sqlite_pool,
            create_sqlite_comparison_files,
            log,
        )
        .await,
        get_changed_rows(
            table_data_1,
            table_data_2,
            &sqlite_pool,
            create_sqlite_comparison_files,
            log,
        )
        .await,
    );

    generate_main_comparison_file(table_data_1, table_data_2, &sqlite_pool, log).await;
    comparison_data
}



/// Get the rows that where the two primary keys match but the other columns differ
async fn get_changed_rows(
    sqlite_table_1: &TableData,
    sqlite_table_2: &TableData,
    sqlite_pool: &SqlitePool,
    create_sqlite_comparison_files: bool,
    log: &Log,
) -> Vec<sqlx::sqlite::SqliteRow> {
    let select_query = if create_sqlite_comparison_files {
        format!("
            create table changedRows_{}
            as
            select *
            from {}
            where exists (
                select * from {} where {} = {}
            );
            select * from changedRows_{}
            ",
            sqlite_table_1.table_name,
            sqlite_table_1.table_name,
            sqlite_table_2.table_name,
            sqlite_table_1.primary_key,
            sqlite_table_2.primary_key,
            sqlite_table_1.table_name
        )
    } else {
        format!("
            select *
            from {}
            where exists (
                select * from {} where {} = {}
            );",
            sqlite_table_1.table_name,
            sqlite_table_2.table_name,
            sqlite_table_1.primary_key,
            sqlite_table_2.primary_key
        )
    }.to_string();

    // execute select query
    let rows = sqlx::query(select_query.as_str())
        .fetch_all(sqlite_pool)
        .await;

    // if no errors return the rows otherwise return that there was an error
    match rows {
        Ok(rows) =>{
            log.info(&format!("extracted {} changed rows", rows.len()));
            rows
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}

/// take the currently generated in flight files and combine them into one
/// table that has all the changes as follows
/// If there is no change then the value remains as follows
/// |     table_column    |
/// |     new value       |
///
/// If there is a value in table 1 not in table 2 it'll display as follows
/// |     table_column    |
/// |     oldValue()      |
///
/// If there is a value in table 2 not in table 1 it'll display as follows
/// |     table_column    |
/// |     ()newValue      |
///
/// If there is a value in table 1 and table 2 but they are different it'll display as follows
/// |     table_column    |
/// | oldValue(newValue)  |
async fn generate_main_comparison_file(
    sqlite_table_1: &TableData,
    sqlite_table_2: &TableData,
    sqlite_pool: &SqlitePool,
    log: &Log,
) -> Vec<sqlx::sqlite::SqliteRow> {
    // initialize the main output query
    let mut comparison_query = format!(
        "create table main_out_{} as select ",
        chrono::offset::Local::now().timestamp()
    );

    // iterate through the columns and generate the query to output the differences in tables
    sqlite_table_1.columns.iter().for_each(|column| {
        let column_name = column.name();
        let query_column = format!(
            "case
                when t1.{} is null and t2.{} is not null then '()'||t2.{}
                when t1.{} is not null and t2.{} is null then t1.{}||'()'
                when t1.{} != t2.{} then t1.{}||'('||t2.{}||')'
            else t1.{}
            end as {},",
            column_name,
            column_name,
            column_name,
            column_name,
            column_name,
            column_name,
            column_name,
            column_name,
            column_name,
            column_name,
            column_name,
            column_name,
        );
        comparison_query.push_str(&query_column);
    });

    comparison_query.pop();
    let changed_rows_join = format!(
        "
        from {} t1
        left join {} t2 on t1.{} = t2.{}
        left join unique_{} new on t1.{} = new.{}
        ",
        sqlite_table_1.table_name,
        sqlite_table_2.table_name,
        sqlite_table_1.primary_key,
        sqlite_table_2.primary_key,
        sqlite_table_1.table_name,
        sqlite_table_1.primary_key,
        sqlite_table_1.primary_key,
    );
    comparison_query.push_str(&changed_rows_join);

    log.info(&format!("comparison query: {}", comparison_query));

    // execute query and return the results
    let rows = sqlx::query(comparison_query.as_str())
        .fetch_all(sqlite_pool)
        .await;

    match rows {
        Ok(rows) => rows,
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}

/// Gets the rows that are unique to the first table and do not eixst in the second
/// If create_sqlite_comparison_files is true then the rows are saved to a new table
/// called unique_{table_name}
async fn get_unique_rows(
    sqlite_table_1: &TableData,
    sqlite_table_2: &TableData,
    sqlite_pool: &SqlitePool,
    create_sqlite_comparison_files: bool,
    log: &Log,
) -> Vec<sqlx::sqlite::SqliteRow> {
    let select_query = if create_sqlite_comparison_files {
        // generate select statement and join on the primary key
        format!(
            "create table unique_{}
            as
            select *
            from {}
            where not exists (
                select * from {} where {} = {}
            );
            select * from unique_{}",
            sqlite_table_1.table_name,
            sqlite_table_1.table_name,
            sqlite_table_2.table_name,
            sqlite_table_1.primary_key,
            sqlite_table_2.primary_key,
            sqlite_table_1.table_name
        )
    } else {
        format!(
            "select *
            from {}
            where not exists (
                select * from {} where {} = {}
            );",
            sqlite_table_2.table_name,
            sqlite_table_1.table_name,
            sqlite_table_1.primary_key,
            sqlite_table_2.primary_key
        )
    };

    // execute select query
    let rows = sqlx::query(select_query.as_str())
        .fetch_all(sqlite_pool)
        .await;

    // if no errors return the rows otherwise return that there was an error
    match rows {
        Ok(rows) => {
            log.info(&format!("extracted {} unique rows", rows.len()));
            rows
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}
