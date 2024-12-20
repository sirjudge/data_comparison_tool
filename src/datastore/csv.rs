use crate::{
    interface::log::Log,
    models::comparison_data::ComparisonData,
    datastore::transformer::sqlite_row_to_string_vec,
};

pub fn export_comparison_data_to_csv(result: &ComparisonData, output_file_name: &str, log: &Log) {
    if !result.unique_table_1_rows.is_empty() {
        let unique_table_1_row_file_name = format!("unique_table_1_rows_{}", output_file_name);
        let mut unique_writer = csv::Writer::from_path(unique_table_1_row_file_name).unwrap();
        for row in result.unique_table_1_rows.iter(){
            let row = sqlite_row_to_string_vec(row, log);
            unique_writer.write_record(row).unwrap();
        }

        unique_writer.flush().unwrap();
        drop(unique_writer);
    }

    if !result.unique_table_2_rows.is_empty() {
        let unique_table_2_row_file_name = format!("unique_table_2_rows_{}", output_file_name);
        let mut unique_writer2 = csv::Writer::from_path(unique_table_2_row_file_name).unwrap();
        for row in result.unique_table_2_rows.iter(){
            let row = sqlite_row_to_string_vec(row, log);
            unique_writer2 .write_record(row).unwrap();
        }
        unique_writer2.flush().unwrap();
        drop(unique_writer2);
    }

    if !result.changed_rows.is_empty() {
        let changed_rows_file_name = format!("changed_rows_{}", output_file_name);
        let mut changed_writer = csv::Writer::from_path(changed_rows_file_name).unwrap();
        for row in result.changed_rows.iter(){
            let row = sqlite_row_to_string_vec(row, log);
            changed_writer.write_record(row).unwrap();
        }
        changed_writer.flush().unwrap();
        drop(changed_writer);
    }
}
