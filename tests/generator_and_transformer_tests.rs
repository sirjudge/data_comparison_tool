use data_comparison_tool::datastore::{
    mysql,
    generator,
    sqlite
};
use async_std::task::block_on;
pub mod setup;


pub async fn generate_mysql_table(table_name: &str) -> usize {
    let (_, log) = setup::setup();

    block_on(data_comparison_tool::datastore::sqlite::drop_table(table_name, &log));
    block_on(generator::create_new_mysql_table_data(100, table_name, &log));

    // check that the table was created
    // check that the table has 100 rows
    // check that the table has the correct columns
    let select_query = format!("select * from {}", table_name);
    let mysql_pool = mysql::get_connection("ComparisonData",&log).await;
    let result = sqlx::query(&select_query).fetch_all(&mysql_pool).await;
    match result {
        Ok(rows) => {
            rows.len()
        },
        Err(error) => {
            panic!("error occurred while fetching rows from mysql table: {:?}", error);
        },
    }
}

/// generates a test table in mysql
#[test]
pub fn generate_mysql_table_test(){
    let table_name = "a_test_table";
    block_on(generate_mysql_table(table_name));
}

/// takes an input test table and copys it to sqlite
#[test]
pub fn copy_mysql_to_sqlite(){
    let (_, log) = setup::setup();
    // generate a test table and extract it into a tableData struct
    let table_name = "b_test_table";
    block_on(mysql::drop_table(table_name, &log));
    let rows_created = block_on(generate_mysql_table(table_name));
    assert_eq!(rows_created, 300);
    let table_data = block_on(mysql::get_table_data(table_name, &log));
    assert_eq!(table_data.columns.len(), 4);

    // query the data itself
    block_on(sqlite::drop_table(table_name, &log));
    let select_query = format!("select * from {}", table_name);
    let sqlite_pool = block_on(mysql::get_connection("ComparisonData", &log));
    let result = block_on(sqlx::query(&select_query).fetch_all(&sqlite_pool));
    match result {
        Ok(rows) => {
            // assert we have exactly 100 rows
            assert_eq!(rows.len(), 100);
        },
        Err(error) => {
            panic!("error occurred while fetching rows from sqlite table: {:?}", error);
        }
    }
}
