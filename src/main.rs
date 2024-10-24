use std::{io, thread, time::Duration};
use async_std::task::block_on;
use data_comparer::ComparisonData;
use std::time::SystemTime;

mod data_querier;
mod data_creator;
mod argument_parser;
mod data_comparer;
mod data_exporter;
mod ui;

fn main() -> Result<(), io::Error>{
    let args = argument_parser::parse_arguments();

    // if help is passed in we want to early return and not do anything else
    // helps prevent people from doing something after pushing the help flag
    if args.help { 
        return Ok(());
    }

    if args.tui {
        let result = ui::run_terminal();
        ratatui::restore();
        return result;
    }
   
    run_comparison(&args);

    // finally return the result
    Ok(())
}

fn run_comparison(args: &argument_parser::Arguments) {
    // if the generate data flag is set then generate the data
    // for the two tables passed in 
    generate_data(args);

    // if the clean flag is set then clean up the sqlite databses
    if args.clean {
        block_on(data_creator::clear_sqlite_data());
        println!("cleaned sqlite database");
    }

    // compare the table data
    let result = compare_data(args); 

    if !args.output_file_name.is_empty() {
        data_exporter::export_data(&result, &args.output_file_name, &args.output_file_type);
    }
}

fn compare_data(args: &argument_parser::Arguments) -> ComparisonData {
    // extract mysql data ino the table data struct
    let table_1_data = block_on(data_querier::get_mysql_table_data(&args.table_name_1));
    let table_2_data = block_on(data_querier::get_mysql_table_data(&args.table_name_2));

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
    let mysql_rows_1= block_on(data_querier::query_mysql(&query_1,database_name ));
    let mysql_rows_2 = block_on(data_querier::query_mysql(&query_2, database_name));
   
    let mut now = SystemTime::now();
    block_on(data_querier::mysql_table_to_sqlite_table(&mysql_rows_1, &table_1_data));
    match now.elapsed(){
        Ok(elapsed) => {
            println!("Time it took to migrate data to sqlite for table 1: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
        }
        Err(e) => { panic!("An error occured: {:?}", e); }
    }

    now = SystemTime::now();
    block_on(data_querier::mysql_table_to_sqlite_table(&mysql_rows_2, &table_2_data));
    match now.elapsed(){
        Ok(elapsed) => {
            println!("Time it took to migrate data to sqlite for table 2: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
        }
        Err(e) => { panic!("An error occured: {:?}", e); }
    }

    // compare the data
    now = SystemTime::now();
    let result = block_on(data_comparer::compare_sqlite_tables(&table_1_data,&table_2_data, args.create_sqlite_comparison_files, args.in_memory_sqlite));
    print_results(&result);
    
    match now.elapsed(){
        Ok(elapsed) => {
            println!("Time it took to compare both tables: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
        }
        Err(e) => { panic!("An error occured: {:?}", e); }
    }


    result
}

/// Prints the results of the comparison in a nice clean fashion
fn print_results(result: &data_comparer::ComparisonData){
    println!("rows in table 1 that are not in table 2: {}", result.unique_table_1_rows.len());
    println!("rows in table 2 that are not in table 1: {}", result.unique_table_2_rows.len());
    println!("rows that are different between the two tables: {}", result.changed_rows.len());
}

/// if args.generate_data is set then generate the data for the two tables
fn generate_data(args: &argument_parser::Arguments){
    if !args.generate_data {
        return
    };

    let mut now = SystemTime::now(); 
    block_on(data_creator::create_new_data(args.number_of_rows_to_generate, &args.table_name_1));
    match now.elapsed(){
        Ok(elapsed) => {
            println!("Time it took to create data: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
        }
        Err(e) => {
            panic!("An error occured: {:?}", e);
        }
    }

    println!("starting second data generation");
    now = SystemTime::now();
    block_on(data_creator::create_new_data(args.number_of_rows_to_generate, &args.table_name_2));
    match now.elapsed(){
        Ok(elapsed) => {
            println!("Time it took to create 2nd table: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
        }
        Err(e) => {
            panic!("An error occured: {:?}", e);
        }
    }
}
