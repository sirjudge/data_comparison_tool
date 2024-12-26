use async_std::task::block_on;
use std::time::SystemTime;
use crate::{
    datastore::{
        mysql,
        sqlite,
        csv,
        generator,
        transformer
    },
    models::comparison_data::ComparisonData,
    interface::{
        log::Log,
        config::{
            Config, OutputFileType
        },
    },
};

pub fn run(config: &Config, log: &Log) -> ComparisonData {
    // if the generate data flag is set then generate the data
    // for the two tables passed in
    if config.data_generation.generate_data {
        generator::generate_data(config, log);
        log.info(&format!("Generated {} rows for each table", config.data_generation.number_of_rows_to_generate));
    }

    // if the clean flag is set then clean up the sqlite databses
    if config.comparison_options.clean {
        block_on(sqlite::clear_sqlite_data());
        log.info("cleaned sqlite database");
    }

    // compare the table data
    let result = compare_data(config, log);

    if !config.comparison_options.output_file_name.is_empty() {
        log.info(&format!("exporting data to file: {}", config.comparison_options.output_file_name));
        match config.comparison_options.output_file_type {
            OutputFileType::Csv => {
                csv::export_comparison_data_to_csv(&result, &config.comparison_options.output_file_name, log);
            }
            OutputFileType::Json => {
                panic!("JSON export not implemented yet");
            }
        }
    }

    result
}

fn compare_data(config: &Config, log: &Log) -> ComparisonData {

    let comp_data_1 =  &config.database_1_config;
    let comp_data_2 =  &config.database_2_config;

    log.debug(&format!("comparing data from {} to {}", comp_data_1.table_name, comp_data_2.table_name));


    // extract mysql data ino the table data struct
    let table_1_data = block_on(mysql::get_table_data(log, &config.database_1_config));
    let table_2_data = block_on(mysql::get_table_data(log, &config.database_2_config));

    // declare query_1 and query_2 variables but don't give them a value
    let mut query_1 = comp_data_1.query.clone();
    let mut query_2 = comp_data_2.query.clone();

    if query_1.is_empty()  {
        query_1 = format!("select * from {}", comp_data_1.table_name);
    }

    if query_2.is_empty()  {
        query_2 = format!("select * from {}", comp_data_2.table_name);
    }

    // OPTIMIZE: this could be either done in parallel or via a stream? row by row.
    // Consider coming back here

    // generate the select statements + return the rows generated from the select statement
    let mysql_rows_1= block_on(mysql::query(&query_1, &comp_data_1, log, config));
    let mysql_rows_2 = block_on(mysql::query(&query_2, &comp_data_2, log, config));

    let mut now = SystemTime::now();
    block_on(transformer::mysql_table_to_sqlite_table(&mysql_rows_1, &table_1_data, log));
    match now.elapsed(){
        Ok(elapsed) => {
            let log_message = format!("Time it took to migrate data to sqlite for table 1: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
            log.info(&log_message);
        }

        Err(e) => {
            panic!("An error occured moving mysql table {} to sqlite: {:?}", table_1_data.table_name ,e);
        }
    }

    now = SystemTime::now();
    block_on(transformer::mysql_table_to_sqlite_table(&mysql_rows_2, &table_2_data, log));
    match now.elapsed(){
        Ok(elapsed) => {
            let log_message = format!("Time it took to migrate data to sqlite for table 2: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
            log.info(&log_message);
        }
        Err(e) => {
            panic!("An error occured moving mysql table {} to sqlite: {:?}", table_2_data.table_name ,e);
        }
    }

    // compare the data
    now = SystemTime::now();
    let result =
        block_on(
            sqlite::compare_tables(
                &table_1_data,
                &table_2_data,
                config.comparison_options.create_sqlite_comparison_files,
                config.comparison_options.in_memory_sqlite,
                log,
                config.log_config.auto_yes
            )
        );

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

