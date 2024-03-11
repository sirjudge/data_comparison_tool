use sqlx::SqlitePool;
use crate::data_querier::{TableData, get_sqlite_connection};

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
   
    new(
        get_unique_rows(table_data_1, table_data_2, &sqlite_pool, create_sqlite_comparison_files).await,
        get_unique_rows(table_data_2, table_data_1, &sqlite_pool, create_sqlite_comparison_files).await,
        get_changed_rows(table_data_1, table_data_2, &sqlite_pool, create_sqlite_comparison_files).await
        ,)
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
                );",
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

/// Gets the rows that are unique to the first table and do not eixst in the second
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
