use data_comparison_tool::datastore::{
    mysql,
    generator,
    sqlite
};
use async_std::task::block_on;
pub mod setup;

/// generates a test table in mysql
#[test]
pub fn generate_mysql_table_test(){
    let table_name = "a_test_table";
    let (_, log) = setup::setup();

    block_on(data_comparison_tool::datastore::sqlite::drop_table(table_name, &log));
    block_on(generator::create_new_mysql_table_data(20, table_name, &log));

    let select_query = format!("select * from {}", table_name);
    let mysql_pool = block_on( mysql::get_connection("ComparisonData",&log));
    let result = block_on(sqlx::query(&select_query).fetch_all(&mysql_pool));
    match result {
        Ok(rows) => {
            // make sure we have the rows populated
            assert!(!rows.is_empty());
        },
        Err(error) => {
            panic!("error occurred while fetching rows from mysql table: {:?}", error);
        },
    }
}

/// takes an input test table and copys it to sqlite
#[test]
pub fn copy_mysql_to_sqlite(){
    let (args, log) = setup::setup();
    // generate a test table and extract it into a tableData struct
    let table_name = "b_test_table";
    block_on(mysql::drop_table(table_name, &log));
    let rows_created = block_on(generate_mysql_table(table_name));
    assert_eq!(rows_created, args.number_of_rows_to_generate as usize);
    let table_data = block_on(mysql::get_table_data(table_name, &log));
    assert_eq!(table_data.columns.len(), 5);

    // query the data itself
    block_on(sqlite::drop_table(table_name, &log));
    let select_query = format!("select * from {}", table_name);
    let sqlite_pool = block_on(mysql::get_connection("ComparisonData", &log));
    let result = block_on(sqlx::query(&select_query).fetch_all(&sqlite_pool));
    match result {
        Ok(rows) => {
            // assert we have exactly 100 rows
            assert_eq!(rows.len(), args.number_of_rows_to_generate as usize);
        },
        Err(error) => {
            panic!("error occurred while fetching rows from sqlite table: {:?}", error);
        }
    }
}
