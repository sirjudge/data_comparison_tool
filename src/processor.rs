use async_std::task::block_on;
use std::time::SystemTime;
use crate::data_creator;
use crate::data_comparer;
use crate::data_querier;
use crate::data_exporter;
use crate::argument_parser;
use crate::data_comparer::ComparisonData;
use crate::log::Log;

pub fn run_comparison(args: &argument_parser::Arguments, log: &Log) -> ComparisonData {
    // if the generate data flag is set then generate the data
    // for the two tables passed in
    generate_data(args, &log);

    // if the clean flag is set then clean up the sqlite databses
    if args.clean {
        block_on(data_creator::clear_sqlite_data());
        log.info("cleaned sqlite database");
    }

    // compare the table data
    let result = compare_data(args, &log);

    if !args.output_file_name.is_empty() {
        data_exporter::export_data(&result, &args.output_file_name, &args.output_file_type, &log);
    }

    result
}

fn compare_data(args: &argument_parser::Arguments, log: &Log) -> ComparisonData {
    // extract mysql data ino the table data struct
    let table_1_data = block_on(data_querier::get_mysql_table_data(&args.table_name_1, log));
    let table_2_data = block_on(data_querier::get_mysql_table_data(&args.table_name_2, log));

    // declare query_1 and query_2 variables but don't give them a value
    let mut query_1 = args.mysql_query_1.clone();
    let mut query_2 = args.mysql_query_2.clone();

    if query_1.is_empty()  {
        query_1 = format!("select * from {}", args.table_name_1);
    }

    if query_2.is_empty()  {
        query_2 = format!("select * from {}", args.table_name_2);
    }

    // generate the select statements + return the rows generated from the select statement
    let database_name = "test";
    let mysql_rows_1= block_on(data_querier::query_mysql(&query_1,database_name, log));
    let mysql_rows_2 = block_on(data_querier::query_mysql(&query_2, database_name, log));

    let mut now = SystemTime::now();
    block_on(data_querier::mysql_table_to_sqlite_table(&mysql_rows_1, &table_1_data));
    match now.elapsed(){
        Ok(elapsed) => {
            let log_message = format!("Time it took to migrate data to sqlite for table 1: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
            log.info(&log_message);
        }

        Err(e) => {
            panic!("An error occured: {:?}", e);
        }
    }

    now = SystemTime::now();
    block_on(data_querier::mysql_table_to_sqlite_table(&mysql_rows_2, &table_2_data));
    match now.elapsed(){
        Ok(elapsed) => {
            let log_message = format!("Time it took to migrate data to sqlite for table 2: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
            log.info(&log_message);
        }
        Err(e) => {
            panic!("An error occured: {:?}", e);
        }
    }

    // compare the data
    now = SystemTime::now();
    let result = block_on(data_comparer::compare_sqlite_tables(&table_1_data,&table_2_data, args.create_sqlite_comparison_files, args.in_memory_sqlite, log));

    match now.elapsed(){
        Ok(elapsed) => {
            log.info(&format!("Time it took to compare both tables: {}.{}", elapsed.as_secs(),elapsed.subsec_millis()));
            log.info(&format!("rows in table 1 that are not in table 2: {}", result.unique_table_1_rows.len()));
            log.info(&format!("rows in table 2 that are not in table 1: {}", result.unique_table_2_rows.len()));
            log.info(&format!("rows that are different between the two tables: {}", result.changed_rows.len()));
        }
        Err(e) => { panic!("An error occured: {:?}", e); }
    }
    result
}

/// if args.generate_data is set then generate the data for the two tables
fn generate_data(args: &argument_parser::Arguments, log: &Log){
    if !args.generate_data {
        log.info("skipping data generation");
        return
    };

    log.info("data creation underway");
    let mut now = SystemTime::now();
    block_on(data_creator::create_new_data(args.number_of_rows_to_generate, &args.table_name_1, log));
    match now.elapsed(){
        Ok(elapsed) => {
            let log_message = format!("Time it took to create data: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
            log.info(&log_message);
        }
        Err(e) => {
            panic!("An error occured: {:?}", e);
        }
    }

    println!("starting second data generation");
    now = SystemTime::now();
    block_on(data_creator::create_new_data(args.number_of_rows_to_generate, &args.table_name_2, &log));
    match now.elapsed(){
        Ok(elapsed) => {
            let log_message = format!("Time it took to create 2nd table: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
            log.info(&log_message);
        }
        Err(e) => {
            panic!("An error occured: {:?}", e);
        }
    }
}
