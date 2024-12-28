use data_comparison_tool::{
    datastore::csv,
    models::comparison_data::ComparisonData
};
pub mod test_utils;

#[test]
fn sqlite_to_csv(){
    //pub fn export(result: &ComparisonData, output_file_name: &str, log: &Log)
    let comparison_data = ComparisonData::empty_data();
    let (_,log) = test_utils::setup();
    let output_file_name = "sqlite_to_csv_test.csv";
    csv::export(&comparison_data, output_file_name, &log);
}
