use crate::data_comparer::ComparisonData;
use crate::argument_parser::OutputFileType;
use csv;
use sqlx::{Type, Row, sqlite::types, Column, sqlite::SqliteTypeInfo, sqlite::types::ValueRef};

pub(crate) fn export_data(result: ComparisonData, output_file_name: &str, output_file_type: OutputFileType){

    println!("exporting data to file: {}", output_file_name);

    match output_file_type {
        OutputFileType::CSV => {
            let mut wtr = csv::Writer::from_path(output_file_name).unwrap();
            for row in result.unique_table_1_rows.iter(){
                let row = sqlite_row_to_csv(row);
                wtr.write_record(row).unwrap();
            }
            
            for row in result.unique_table_2_rows.iter(){
                let row = sqlite_row_to_csv(row);
                wtr.write_record(row).unwrap();
            }

            for row in result.changed_rows.iter(){
                let row = sqlite_row_to_csv(row);
                wtr.write_record(row).unwrap();
            }

            wtr.flush().unwrap();
        }
        OutputFileType::JSON => {
            panic!("JSON export not implemented yet");
        }
    }
}

fn sqlite_row_to_csv(row: &sqlx::sqlite::SqliteRow) -> Vec<String> {
    let mut row_string = Vec::new();
    for i in 0..row.len(){
        let value = row.get(i);
        //BUG: believe this is't correctly handling column types for some odd reason
        let value = match value {
            sqlx::sqlite::types::ValueRef::Null => "NULL".to_string(),
            sqlx::sqlite::types::ValueRef::Integer(i) => i.to_string(),
            sqlx::sqlite::types::ValueRef::Real(r) => r.to_string(),
            sqlx::sqlite::types::ValueRef::Text(t) => t.to_string(),
            sqlx::sqlite::types::ValueRef::Blob(b) => b.to_string(),
        };
        row_string.push(value);
    }
    row_string
}
