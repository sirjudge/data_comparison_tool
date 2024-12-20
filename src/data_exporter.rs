use crate::{
    log::Log,
    models::{
        argument_parser::OutputFileType,
        comparison_data::ComparisonData
    },
};
use sqlx::{ Row, Column  };

pub(crate) fn export_data(result: &ComparisonData, output_file_name: &str, output_file_type: &OutputFileType, log: &Log) {

    log.info(&format!("exporting data to file: {}", output_file_name));

    match output_file_type {
        OutputFileType::Csv => {
            if !result.unique_table_1_rows.is_empty() {
                let unique_table_1_row_file_name = format!("unique_table_1_rows_{}", output_file_name);
                let mut unique_writer = csv::Writer::from_path(unique_table_1_row_file_name).unwrap();
                for row in result.unique_table_1_rows.iter(){
                    let row = sqlite_row_to_csv(row, log);
                    unique_writer.write_record(row).unwrap();
                }

                unique_writer.flush().unwrap();
                drop(unique_writer);
            }

            if !result.unique_table_2_rows.is_empty() {
                let unique_table_2_row_file_name = format!("unique_table_2_rows_{}", output_file_name);
                let mut unique_writer2 = csv::Writer::from_path(unique_table_2_row_file_name).unwrap();
                for row in result.unique_table_2_rows.iter(){
                    let row = sqlite_row_to_csv(row, log);
                    unique_writer2 .write_record(row).unwrap();
                }
                unique_writer2.flush().unwrap();
                drop(unique_writer2);
            }

            if !result.changed_rows.is_empty() {
                let changed_rows_file_name = format!("changed_rows_{}", output_file_name);
                let mut changed_writer = csv::Writer::from_path(changed_rows_file_name).unwrap();
                for row in result.changed_rows.iter(){
                    let row = sqlite_row_to_csv(row, log);
                    changed_writer.write_record(row).unwrap();
                }
                changed_writer.flush().unwrap();
                drop(changed_writer);
            }
        }
        OutputFileType::Json => {
            panic!("JSON export not implemented yet");
        }
    }
}

fn sqlite_row_to_csv(row: &sqlx::sqlite::SqliteRow, log: &Log) -> Vec<String> {
    // convert sqliteRow to csv row
    let mut csv_row = Vec::new();
    let number_of_columns = row.columns().len();
    for i in 0..number_of_columns {
        let column_type = row.column(i).type_info().to_string();
        match column_type.as_str() {
            "TEXT" => {
                let value: String = row.get(i);
                csv_row.push(value);
            }
            "INTEGER" => {
                let value: i64 = row.get(i);
                csv_row.push(value.to_string());
            }
            "REAL" => {
                let value: f64 = row.get(i);
                csv_row.push(value.to_string());
            }
            "BLOB" => {
                let value: String = row.get(i);
                csv_row.push(value);
            }
            _ => {
                log.error(&format!("unknown column type: {}", column_type));
            }
        }
    }
    // finally return csv row
    csv_row
}

