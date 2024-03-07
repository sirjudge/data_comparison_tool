use sqlx::SqlitePool;
use crate::data_querier::{TableData, get_sqlite_connection};

pub struct ComparisonData {
    pub unique_table_1_rows: Vec<sqlx::sqlite::SqliteRow>,
    pub unique_table_2_rows: Vec<sqlx::sqlite::SqliteRow>,
    pub changed_rows: Vec<sqlx::sqlite::SqliteRow>,
    pub table_1_data: TableData,
    pub table_2_data: TableData,
}

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
    table_data_2: &TableData) -> ComparisonData {
    let sqlite_pool = get_sqlite_connection().await;
    let sqlite_rows_1 = get_unique_rows(table_data_1, table_data_2, &sqlite_pool).await;
    let sqlite_rows_2 = get_unique_rows(table_data_2, table_data_1, &sqlite_pool).await;
    let changed_rows = get_changed_rows(table_data_1, table_data_2, &sqlite_pool).await;
    new(sqlite_rows_1, sqlite_rows_2,changed_rows,)
}

/// Get the rows that where the two primary keys match but the other columns differ
async fn get_changed_rows(
    sqlite_table_1: &TableData,
    sqlite_table_2: &TableData,
    sqlite_pool: &SqlitePool
    ) -> Vec<sqlx::sqlite::SqliteRow> {
    // generate a select statement to find rows that have the same primary id but differ in other columns
    let select_query = format!(
        "select * from {} where exists (select * from {} where {} = {})",
        sqlite_table_1.table_name,
        sqlite_table_2.table_name,
        sqlite_table_1.primary_key,
        sqlite_table_2.primary_key);

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
    sqlite_pool: &SqlitePool
    ) -> Vec<sqlx::sqlite::SqliteRow> {
    // generate select statement and join on the primary key 
    let select_query = format!(
        "select * from {} where not exists (select * from {} where {} = {})",
        sqlite_table_1.table_name,
        sqlite_table_2.table_name,
        sqlite_table_1.primary_key,
        sqlite_table_2.primary_key);

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

