

pub(crate) asyn fn compare_sqlite_tables(sqlite_table1: &str, sqlite_table2: &str) -> bool {
    let sqlite_pool = get_sqlite_connection().await;
    let query = format!("SELECT * FROM {} EXCEPT SELECT * FROM {}", sqlite_table1, sqlite_table2);
    let result = sqlx::query(query.as_str())
        .fetch_all(&sqlite_pool)
        .await;
    match result {
        Ok(rows) => {
            if rows.is_empty() {
                return true;
            }
            else {
                return false;
            }
        }
        Err(error) => {
            panic!("error occurred while comparing sqlite tables: {:?}", error);
        }
    }
}
