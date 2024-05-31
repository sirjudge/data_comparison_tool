//export_data(&result, &args.output_file_name, args.output_file_type);
use crate::data_comparer::ComparisonData;
use crate::argument_parser::OutputFileType;
use csv::Writer;
use sqlx::Row;

pub(crate) fn export_data(result: ComparisonData, output_file_name: &str, output_file_type: OutputFileType){
    match output_file_type {
        OutputFileType::CSV => {
            let mut wtr = csv::Writer::from_path(output_file_name).unwrap();
            for row in result.unique_table_1_rows.iter(){
                let row = sqlite_row_to_csv(row);
                for i in 0..row.len(){
                    wtr.write_record(&[row[i].to_string(), "Unique to Table 1".to_string()]);
                } 
            }
            wtr.flush().unwrap();
        }
        OutputFileType::JSON => {
            //TODO: Implement JSON export
        }
    }
}


fn sqlite_row_to_csv(row: &sqlx::sqlite::SqliteRow) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..row.len(){
            
    }
    result
}
