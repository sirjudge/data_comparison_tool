use crate::{
    interface::{
        log::Log,
        argument_parser::OutputFileType,
    },
    datastore::csv::export_to_csv,
    models::comparison_data::ComparisonData,
};

pub(crate) fn export_data(result: &ComparisonData, output_file_name: &str, output_file_type: &OutputFileType, log: &Log) {
    log.info(&format!("exporting data to file: {}", output_file_name));
    match output_file_type {
        OutputFileType::Csv => {
            export_to_csv(result, output_file_name, log);
        }
        OutputFileType::Json => {
            panic!("JSON export not implemented yet");
        }
    }
}


