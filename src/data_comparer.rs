use sqlx::SqlitePool;

use crate::data_querier;

pub(crate) async fn compare_sqlite_tables(sqlite_table1: &str, sqlite_table2: &str) -> bool {
    let sqlite_pool = data_querier::get_sqlite_connection().await;
    let sqlite_rows_1 = get_unique_rows_from_table_1(sqlite_table1, sqlite_table2, &sqlite_pool).await;
    let sqlite_rows_2 = get_unique_rows_from_table_2(sqlite_table1, sqlite_table2, &sqlite_pool).await;
    return true;
}

async fn get_unique_rows_from_table_1(table1: &str, table2: &str, sqlite_pool: &SqlitePool) -> Vec<sqlx::mysql::MySqlRow> {
    let query = format!("SELECT * FROM {} EXCEPT SELECT * FROM {}", table1, table2);
    let rows = data_querier::query_mysql(query.as_str()).await;
    println!("there are {} unique rows in table 2", rows.len());
    return rows;
}

async fn get_unique_rows_from_table_2(table1: &str, table2: &str, sqlite_pool: &SqlitePool) -> Vec<sqlx::mysql::MySqlRow> {
    let query = format!("SELECT * FROM {} EXCEPT SELECT * FROM {}", table2, table1);
    let rows = data_querier::query_mysql(query.as_str()).await;
    println!("there are {} unique rows in table 2", rows.len());
    return rows;
}
