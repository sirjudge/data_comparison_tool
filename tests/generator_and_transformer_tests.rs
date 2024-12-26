use data_comparison_tool::datastore::{
    mysql,
    generator,
    sqlite
};
use async_std::task::block_on;
pub mod test_utils;

/// generates a test table in mysql
#[test]
pub fn generate_mysql_table(){
    let (config, log) = test_utils::setup();
    let table_name = config.database_1_config.table_name.as_str();
    block_on(data_comparison_tool::datastore::sqlite::drop_table(table_name, &log));
    block_on(
        generator::generate_table(&config, &log, &config.database_1_config)
    );

    let select_query = format!("select * from {}", table_name);
    let mysql_pool = block_on( mysql::get_connection(&log, &config.database_1_config));
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
     let (config, log) = test_utils::setup();
     let table_name = config.database_1_config.table_name.as_str();

     // generate a test table and extract it into a tableData struct
     let rows_created = block_on(
         test_utils::generate_mysql_table(&config, table_name, &config.database_1_config)
     );

     //BUG: expecteing rows_created to be 20 but it's 40 instead? da heck?
     assert_eq!(rows_created, config.data_generation.number_of_rows_to_generate as usize);
     let table_data = block_on(mysql::get_table_data(&log, &config.database_1_config));
     assert_eq!(table_data.columns.len(), 5);

     // query the data itself
     block_on(sqlite::drop_table(table_name, &log));
     let select_query = format!("select * from {}", table_name);
     let sqlite_pool = block_on(mysql::get_connection(&log, &config.database_2_config));
     let result = block_on(sqlx::query(&select_query).fetch_all(&sqlite_pool));
     match result {
         Ok(rows) => {
             // assert we have exactly 100 rows
             assert_eq!(rows.len(), config.data_generation.number_of_rows_to_generate as usize);
         },
         Err(error) => {
             panic!("error occurred while fetching rows from sqlite table: {:?}", error);
         }
     }
}
