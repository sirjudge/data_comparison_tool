use data_comparison_tool::{
    datastore::generator,
};
use async_std::task::block_on;
pub mod setup;

#[test]
pub fn generate_mysql_table(){
    let (args, log) = setup::setup();
    block_on(
        generator::create_new_mysql_table_data(args.number_of_rows_to_generate, &args.table_name_2, &log)
    );
}

#[test]
pub fn generate_sqlite_table(){
    panic!("not implemented");
}

#[test]
pub fn copy_mysql_to_sqlite(){
    panic!("not implemented");
}
