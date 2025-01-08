use crate::{
    interface::log::Log,
    models::comparison_data::ComparisonData,
};
use sqlx::{
    sqlite::SqliteRow,
    Column,
    Row,
};

pub fn sqlite_row_to_string_vec(row:&SqliteRow, log: &Log) -> Vec<String> {
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

fn write_sqlite_vec_to_file(
    sqlite_rows: &[SqliteRow],
    log: &Log,
    output_file_name: &str,

){
    if sqlite_rows.is_empty(){
        log.warn(&format!("Sqlite rows passed in are empty, no need to log to {}",output_file_name));
        return;
    }

    if !sqlite_rows.is_empty() {
        // open csv writer
        let unique_table_1_row_file_name =
            format!("unique_table_1_rows_{}", output_file_name);
        let mut unique_writer =
            csv::Writer::from_path(unique_table_1_row_file_name).unwrap();

        // extract the rows and write each line
        for row in sqlite_rows.iter(){
            let row = sqlite_row_to_string_vec(row, log);
            unique_writer.write_record(row).unwrap();
        }

        // flush the current buffer to file and drop the writer
        unique_writer.flush().unwrap();
        drop(unique_writer);
    }
}

/// takes a given ComparisonData object and extracts it to 0 - 3
/// files if the given input data is a non empty Vec
pub fn export(result: &ComparisonData, output_file_name: &str, log: &Log) {

    write_sqlite_vec_to_file(&result.unique_table_1_rows,log, output_file_name);
    write_sqlite_vec_to_file(&result.unique_table_2_rows,log, output_file_name);
    write_sqlite_vec_to_file(&result.changed_rows,log, output_file_name);
}
