use sqlx::mysql::MySqlColumn;


/// Struct to hold the table properties to pass over to the sqlite querier
pub struct TableData {
    /// name of the table you're querying
    pub table_name: String,
    /// list of columns in the table
    pub columns: Vec<MySqlColumn>,
    /// primary key of the table we're joinin on
    pub primary_key: String,
}

impl TableData {
    pub fn new(table_name: String, columns: Vec<MySqlColumn>, primary_key: String) -> TableData {
        TableData {
            table_name,
            columns,
            primary_key,
        }
    }
}
