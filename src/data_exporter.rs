use crate::data_comparer::ComparisonData;
use crate::argument_parser::OutputFileType;
//use csv;
use sqlx::{ Row, sqlite::{types, SqliteColumn, SqliteTypeInfo}, Column  };

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

            println!("Exported data to file: {}", output_file_name);
            wtr.flush().unwrap();
        }
        OutputFileType::JSON => {
            panic!("JSON export not implemented yet");
        }
    }
}

fn sqlite_row_to_csv(row: &sqlx::sqlite::SqliteRow) -> Vec<String> {
    // convert sqliteRow to csv row
    let mut csv_row = Vec::new();
    let number_of_columns = row.columns().len();
    for i in 0..number_of_columns {
        let column_type = row.column(i).type_info().to_string();
        let column_name = row.column(i).name().to_string();
        println!("column_name: {}", column_name);
        println!("column_name: {}", column_type);
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
                //TODO:Figure this out later?
            }
            _ => {
                println!("unknown column type: {}", column_type);
            }
        }
    }
    // finally return csv row
    csv_row
}

