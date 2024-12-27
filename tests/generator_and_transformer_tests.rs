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
    block_on(generator::generate_table(&config, &log, &config.database_1_config));

    // query the data using the db1 config
    let select_query = format!("select * from {}", config.database_1_config.table_name);
    let mysql_pool = block_on( mysql::get_connection(&log, &config.database_1_config));
    let result = block_on(sqlx::query(&select_query).fetch_all(&mysql_pool));

    // assert on the restult
    match result {
        Ok(rows) => {
            // assert that rows is not empty
            assert!(!rows.is_empty(), "no rows were generated in the mysql table: {}", table_name);

            // assert that we have the same amount of rows
            // as we expected to generate
            assert_eq!(
                rows.len(),
                config.data_generation.number_of_rows_to_generate as usize,
                "expected {} rows but got {} rows", config.data_generation.number_of_rows_to_generate, rows.len()
            );

            // if all is well in the neighborhood, log success if
            // we're in debug mode
            log.debug(
                &format!("successfully generated {} rows in the mysql table", rows.len())
            );

            // get table data and make sure comlumns are set correctly
            let table_data = block_on(mysql::get_table_data(&log, &config.database_1_config));
            assert_eq!(table_data.columns.len(), 5);
        },
        Err(error) => {
            // if something happend panic the error
            panic!("error occurred while fetching rows from mysql table: {:?}", error);
        },
    }
}

/// takes an input test table and copys it to sqlite
#[test]
pub fn copy_mysql_to_sqlite(){
    // init test data
    let (mut config, log) = test_utils::setup();
    let table_name = "sqlite_copy_table";
    config.database_1_config.table_name = table_name.to_string();

    // generate a test table and extract it into a tableData struct
    let _ = block_on(
        generator::generate_table(&config, &log, &config.database_1_config)
    );

    // select from the table
    let select_query = format!("select * from {}", table_name);
    let sqlite_pool = block_on(mysql::get_connection(&log, &config.database_2_config));
    let result = block_on(sqlx::query(&select_query).fetch_all(&sqlite_pool));
    match result {
        Ok(rows) => {
            // assert we have exactly 100 rows
            // BUG: THIS IS FAILLING DANG IT
            assert_eq!(rows.len(), config.data_generation.number_of_rows_to_generate as usize);
        },
        Err(error) => {
            panic!("error occurred while fetching rows from sqlite table: {:?}", error);
        }
    }
}
