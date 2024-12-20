

/// Struct to hold the comparison data between the two tables
pub struct ComparisonData {
    /// Rows that are unique to the first table and do not exist in the second
    pub unique_table_1_rows: Vec<sqlx::sqlite::SqliteRow>,

    /// Rows that are unique to the second table and do not exist in the first
    /// table
    pub unique_table_2_rows: Vec<sqlx::sqlite::SqliteRow>,

    /// Rows that have the same primary key but differ in other columns
    pub changed_rows: Vec<sqlx::sqlite::SqliteRow>,
}

impl ComparisonData {
    /// Constructor for the comparison data struct
    pub fn new(
        unique_table_1_data: Vec<sqlx::sqlite::SqliteRow>,
        unique_table_2_data: Vec<sqlx::sqlite::SqliteRow>,
        changed_rows_data: Vec<sqlx::sqlite::SqliteRow>,
    ) -> ComparisonData {
        // return the new comparison object
        ComparisonData {
            unique_table_1_rows: unique_table_1_data,
            unique_table_2_rows: unique_table_2_data,
            changed_rows: changed_rows_data,
        }
    }
}

