use sqlx::SqlitePool;
use crate::data_querier::TableData;
use crate::data_querier;


pub struct ComparisonData {
    pub unique_table_1_rows: Vec<sqlx::sqlite::SqliteRow>,
    pub unique_table_2_rows: Vec<sqlx::sqlite::SqliteRow>,
    pub table_1_data: TableData,
    pub table_2_data: TableData,
    pub comparison_result: bool
}


pub(crate) async fn compare_sqlite_tables(table_data_1: &TableData, table_data_2: &TableData) -> bool {
    let sqlite_pool = data_querier::get_sqlite_connection().await;
    let sqlite_rows_1 = get_unique_rows(table_data_1, table_data_2, &sqlite_pool).await;
    let sqlite_rows_2 = get_unique_rows(table_data_2, table_data_1, &sqlite_pool).await;

    println!("rows in table 1 that are not in table 2: {}", sqlite_rows_1.len());
    println!("rows in table 2 that are not in table 1: {}", sqlite_rows_2.len());

    return true;
}

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
            return rows;
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}

