use sqlx::{Column, SqlitePool};
use crate::data_querier::{TableData, get_sqlite_connection};
use chrono;


/// Struct to hold the comparison data between the two tables
pub struct ComparisonData {
    /// Rows that are unique to the first table and do not exist in the second
    /// table
    pub unique_table_1_rows: Vec<sqlx::sqlite::SqliteRow>,
    /// Rows that are unique to the second table and do not exist in the first
    /// table
    pub unique_table_2_rows: Vec<sqlx::sqlite::SqliteRow>,

    /// Rows that have the same primary key but differ in other columns
    pub changed_rows: Vec<sqlx::sqlite::SqliteRow>,

    /// The table data for the first table
    pub table_1_data: TableData,

    /// The table data for the second table
    pub table_2_data: TableData,
}

/// Constructor for the comparison data struct
fn new (
    unique_table_1_data: Vec<sqlx::sqlite::SqliteRow>,
    unique_table_2_data: Vec<sqlx::sqlite::SqliteRow>,
    changed_rows_data: Vec<sqlx::sqlite::SqliteRow>,
    ) -> ComparisonData {

    // initialize the table data objects
    let t1_data = TableData {
        table_name: String::from(""),
        primary_key: String::from(""),
        columns: Vec::new(),
    };
    let t2_data =  TableData {
        table_name: String::from(""),
        primary_key: String::from(""),
        columns: Vec::new(),
    };

    // return the new comparison object
    ComparisonData {
        unique_table_1_rows: unique_table_1_data,
        unique_table_2_rows: unique_table_2_data,
        changed_rows: changed_rows_data,
        table_1_data: t1_data,
        table_2_data: t2_data
    }
}

/// Compare two sqlite tables and return the differences
pub(crate) async fn compare_sqlite_tables(
    table_data_1: &TableData,
    table_data_2: &TableData,
    mut create_sqlite_comparison_files: bool,
    in_memory_sqlite: bool
    ) -> ComparisonData {

    if in_memory_sqlite && create_sqlite_comparison_files {
        println!("using in memory sqlite for data comparison,
                 this will be faster but will not save the comparison
                 data to disk, do you want to continue? (yes/no)");
        // read from std in
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        //TODO: take in input args auto-yes flag to pass down to here
        if input == "yes\n" || input == "y" {
            println!("continuing with in memory sqlite");
            create_sqlite_comparison_files = false;
        }
        else {
            println!("exiting program");
            std::process::exit(0);
        }
    }

    // get the sqlite connection, and execute each part of the comparison
    let sqlite_pool = get_sqlite_connection().await;

    let comparison_data = new (
        get_unique_rows(table_data_1, table_data_2, &sqlite_pool, create_sqlite_comparison_files).await,
        get_unique_rows(table_data_2, table_data_1, &sqlite_pool, create_sqlite_comparison_files).await,
        get_changed_rows(table_data_1, table_data_2, &sqlite_pool, create_sqlite_comparison_files).await,
        );

    generate_main_comparison_file(table_data_1, table_data_2, &sqlite_pool).await;
    comparison_data
}

/// Get the rows that where the two primary keys match but the other columns differ
async fn get_changed_rows(
    sqlite_table_1: &TableData,
    sqlite_table_2: &TableData,
    sqlite_pool: &SqlitePool,
    create_sqlite_comparison_files: bool
    ) -> Vec<sqlx::sqlite::SqliteRow> {
    // generate a select statement to find rows that have the same primary id but differ in other columns

    let select_query;
    if create_sqlite_comparison_files {
        select_query = format!(
            "create table changedRows_{}
            as
            select *
            from {}
            where exists (
                select * from {} where {} = {}
                );
            select * from changedRows_{}",
            sqlite_table_1.table_name,
            sqlite_table_1.table_name,
            sqlite_table_2.table_name,
            sqlite_table_1.primary_key,
            sqlite_table_2.primary_key,
            sqlite_table_1.table_name);
    }
    else {
        select_query = format!(
            "
            select *
            from {}
            where exists (
                select * from {} where {} = {}
                );
            ",
            sqlite_table_1.table_name,
            sqlite_table_2.table_name,
            sqlite_table_1.primary_key,
            sqlite_table_2.primary_key);
    }

    // execute select query
    let rows = sqlx::query(select_query.as_str())
        .fetch_all(sqlite_pool)
        .await;

    // if no errors return the rows otherwise return that there was an error
    match rows {
        Ok(rows) => {
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
async fn generate_main_comparison_file(sqlite_table_1: &TableData, sqlite_table_2: &TableData, sqlite_pool: &SqlitePool) -> Vec<sqlx::sqlite::SqliteRow>{
    // TODO: Figure out if I actually need this still or not
    // extract timestamp from table name
    /*
    let table_name_split = sqlite_table_1.table_name.split('_');
    let table_name_vec: Vec<&str> = table_name_split.collect();
    let time_stamp = table_name_vec[1];
    */

    // initialize the main output query
    let mut comparison_query = format!("create table main_out_{} as select ", chrono::offset::Local::now().timestamp());

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

    println!("comparison query: {}", comparison_query);

    // execute query and return the results
    let rows = sqlx::query(comparison_query.as_str())
        .fetch_all(sqlite_pool)
        .await;

    match rows {
        Ok(rows) => {
            rows
        }
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
    create_sqlite_comparison_files: bool
    ) -> Vec<sqlx::sqlite::SqliteRow> {

    let select_query;
    if create_sqlite_comparison_files{
        // generate select statement and join on the primary key
        select_query = format!(
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
            sqlite_table_1.table_name);
    }
    else {
        select_query = format!(
            "select *
            from {}
            where not exists (
                select * from {} where {} = {}
            );",
            sqlite_table_2.table_name,
            sqlite_table_1.table_name,
            sqlite_table_1.primary_key,
            sqlite_table_2.primary_key);
    }

    // execute select query
    let rows = sqlx::query(select_query.as_str())
        .fetch_all(sqlite_pool)
        .await;

    // if no errors return the rows otherwise return that there was an error
    match rows {
        Ok(rows) => {
            rows
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}
