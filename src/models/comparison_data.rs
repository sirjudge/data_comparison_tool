use core::panic;

use sqlx::Row;

/// Struct to hold the comparison data between the two tables
#[derive(Clone)]
pub struct ComparisonData {
    /// Rows that are unique to the first table and do not exist in the second
    pub unique_table_1_rows: Vec<ComparisonRow>,

    /// Rows that are unique to the second table and do not exist in the first
    /// table
    pub unique_table_2_rows: Vec<ComparisonRow>,

    /// Rows that have the same primary key but differ in other columns
    pub changed_rows: Vec<ChangedRow>,
}

#[derive(Clone)]
pub struct ComparisonRow {
    pub row: Vec<String>,
}

#[derive(Clone)]
pub struct ChangedRow {
    pub table_1_row: Vec<String>,
    pub table_2_row: Vec<String>,
}

impl ComparisonData {
    /// Constructor for the comparison data struct
    pub fn new() -> ComparisonData {
        ComparisonData {
            unique_table_1_rows: Vec::new(),
            unique_table_2_rows: Vec::new(),
            changed_rows: Vec::new(),
        }
    }

    /// intializes each comparison data as empty data to handle edge cases
    /// where a comparison is not possible or if the tables are completely
    /// the same
    pub fn empty_data() -> ComparisonData{
        ComparisonData {
            unique_table_1_rows: Vec::new(),
            unique_table_2_rows :Vec::new(),
            changed_rows : Vec::new()
        }
    }

    pub fn from_row(row: &sqlx::mysql::MySqlRow) -> ComparisonRow {
        let mut comparison_row = ComparisonRow {
            row: Vec::new(),
        };

        // Convert each column to a string and add it to the row vector
        for i in 0..row.len() {
            if let Ok(value) = row.try_get::<String, _>(i) {
                comparison_row.row.push(value);
            } else {
                // comparison_row.row.push("NULL".to_string());
                // TODO: Not sure if should panic or not here
                // Ye ol powerful AI wanted to just push a string literal "NULL" as seen
                // above but I am not convinced
                panic!("Failed to convert column {} to string", i);
            }
        }

        comparison_row
    }

    // Add a method to convert SQLite rows to our new format
    pub fn from_sqlite_comparison(
        unique_table_1: Vec<ComparisonRow>,
        unique_table_2: Vec<ComparisonRow>,
        changed: Vec<(ComparisonRow, ComparisonRow)>
    ) -> Self {
        ComparisonData {
            unique_table_1_rows: unique_table_1,
            unique_table_2_rows: unique_table_2,
            changed_rows: changed.into_iter()
                .map(|(r1, r2)| ChangedRow {
                    table_1_row: r1.row,
                    table_2_row: r2.row,
                })
                .collect()
        }
    }
}
