use crate::{
    interface::{
        log::Log,
        config::Config
    },
    models::{
        comparison_data::{ComparisonData, ComparisonRow},
        table_data::TableData,
    }
};
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::SqlitePoolOptions,
    SqlitePool,
    Column,
    Row,
    Pool,
};

/// open a connection to the sqlite database
pub async fn get_connection(log: &Log) -> Pool<sqlx::Sqlite> {
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

pub async fn drop_table(table_name: &str, log: &Log) {
    // get the sqlite connection and create the query
    // to check if the table exists
    let sqlite_pool = get_connection(log).await;
    let check_if_table_exists_query = format!(
        "select count(*) from sqlite_master where type='table' and name='{}'",
        table_name
    );

    // query the db and if the table doesn't exist return early else
    // run the drop table and check if we successfully dropped
    let result = sqlx::query(check_if_table_exists_query.as_str())
        .fetch_one(&sqlite_pool)
        .await;
    match result {
        Ok(row) => {
            let count: i32 = row.get(0);
            if count == 0 {
                return;
            }

            let drop_query = format!("drop table {}", table_name);
            let result = sqlx::query(drop_query.as_str())
                .execute(&sqlite_pool)
                .await;

            match result {
                Ok(_) => {
                    log.info(&format!("dropped table: {}", table_name));
                },
                Err(error) => {
                    panic!("error occurred while dropping table: {:?}", error);
                },
            }
        },
        Err(error) => {
            panic!("error occurred while checking if table exists: {:?}", error);
        },
    }
}

/// Compare two sqlite tables and return the differences
pub async fn compare_tables(
    table_1_data: &TableData,
    table_2_data: &TableData,
    log: &Log,
    config: &Config,
) -> ComparisonData {
    let sqlite_pool = self::get_connection(log).await;
    let create_sqlite_files = config.comparison_options.create_sqlite_comparison_files;
    
    // Get all the data first
    let unique_1 = get_unique_rows(table_1_data, table_2_data, &sqlite_pool, create_sqlite_files, log).await;
    let unique_2 = get_unique_rows(table_2_data, table_1_data, &sqlite_pool, create_sqlite_files, log).await;
    let changed = get_changed_rows(table_1_data, table_2_data, &sqlite_pool, create_sqlite_files, log).await;

    generate_main_comparison_file(table_1_data, table_2_data, &sqlite_pool, log).await;

    // Convert the results using the new method
    ComparisonData::from_sqlite_comparison(unique_1, unique_2, changed)
}

/// Get the rows that where the two primary keys match but the other columns differ
pub async fn get_changed_rows(
    sqlite_table_1: &TableData,
    sqlite_table_2: &TableData,
    sqlite_pool: &SqlitePool,
    create_sqlite_comparison_files: bool,
    log: &Log,
) -> Vec<(ComparisonRow, ComparisonRow)> {
    let select_query = if create_sqlite_comparison_files {
        let changed_table_name = format!("changedRows_{}", sqlite_table_1.table_name);
        if check_if_table_exists(&changed_table_name, sqlite_pool).await {
            log.warn(&format!("table {} already exists, dropping table", changed_table_name));
            drop_table(&changed_table_name, log).await;
        }

        format!("
            select t1.*, t2.*
            from {} t1
            join {} t2 on t1.{} = t2.{}
            where not ({})",
            sqlite_table_1.table_name,
            sqlite_table_2.table_name,
            sqlite_table_1.primary_key,
            sqlite_table_2.primary_key,
            sqlite_table_1.columns.iter()
                .map(|col| format!("t1.{} = t2.{}", col.name(), col.name()))
                .collect::<Vec<_>>()
                .join(" and ")
        )
    } else {
        format!("
            select t1.*, t2.*
            from {} t1
            join {} t2 on t1.{} = t2.{}
            where not ({})",
            sqlite_table_1.table_name,
            sqlite_table_2.table_name,
            sqlite_table_1.primary_key,
            sqlite_table_2.primary_key,
            sqlite_table_1.columns.iter()
                .map(|col| format!("t1.{} = t2.{}", col.name(), col.name()))
                .collect::<Vec<_>>()
                .join(" and ")
        )
    };

    let rows = sqlx::query(&select_query)
        .fetch_all(sqlite_pool)
        .await;

    match rows {
        Ok(rows) => {
            let row_count = rows.len();
            log.info(&format!("found {} changed rows", row_count));
            
            let mut changed_pairs = Vec::with_capacity(row_count);
            for row in rows {
                let col_count = sqlite_table_1.columns.len();
                let mut row1_values = Vec::new();
                let mut row2_values = Vec::new();

                // Extract values for first table
                for i in 0..col_count {
                    let value: String = row.get(i);
                    row1_values.push(value);
                }

                // Extract values for second table
                for i in col_count..(col_count * 2) {
                    let value: String = row.get(i);
                    row2_values.push(value);
                }

                // Create ComparisonRow structs
                let row1 = ComparisonRow { row: row1_values };
                let row2 = ComparisonRow { row: row2_values };
                
                changed_pairs.push((row1, row2));
            }
            changed_pairs
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
                when t1.{} is null and t2.{} is not null then '()'||t2.{}  // Value exists in t2 but not in t1
                when t1.{} is not null and t2.{} is null then t1.{}||'()'  // Value exists in t1 but not in t2
                when t1.{} != t2.{} then t1.{}||'('||t2.{}||')'  // Values differ
                else t1.{}  // Values are the same
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

    log.debug(&format!("comparison query: {}", comparison_query));

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

pub async fn check_if_table_exists(table_name: &str, sqlite_pool: &SqlitePool) -> bool {
    let check_if_table_exists_query = format!(
        "select count(*) from sqlite_master where type='table' and name='{}'",
        table_name
    );

    let result = sqlx::query(check_if_table_exists_query.as_str())
        .fetch_one(sqlite_pool)
        .await;

    match result {
        Ok(row) => {
            let count: i32 = row.get(0);
            count > 0
        },
        Err(error) => {
            panic!("error occurred while checking if table exists: {:?}", error);
        },
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
) -> Vec<ComparisonRow> {
    let select_query = if create_sqlite_comparison_files {
        //TODO: This works for now to get it stable but should come back and revisit
        // this at some point to handle a bit more gracefully than whoops lol your prev
        // data is gone
        if check_if_table_exists(&sqlite_table_1.table_name, sqlite_pool).await {
            log.warn(&format!("table {} already exists, dropping table", sqlite_table_1.table_name));
            drop_table(&format!("unique_{}", sqlite_table_1.table_name), log).await;
        }

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
            // Convert SqliteRows to ComparisonRows
            rows.into_iter().map(|row| {
                let mut values = Vec::new();
                for i in 0..row.len() {
                    let value: String = row.get(i);
                    values.push(value);
                }
                ComparisonRow { row: values }
            }).collect()
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}

/// Cleans up all sqlite files inside the current executing directory
pub async fn clear_sqlite_data(){
    // get all files in the current directory
    let files = std::fs::read_dir(".").unwrap();
    for file in files{
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name.ends_with(".sqlite"){
            std::fs::remove_file(file_name).unwrap();
        }
    }
}

