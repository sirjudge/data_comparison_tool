use async_std::task::block_on;
use std::time:: {SystemTime};

mod data_querier;
mod data_creator;
mod argument_parser;
mod data_comparer;

fn main() {
    let args = argument_parser::parse_arguments();


    // create new amount of random data
    if args.generate_data {
        println!("Starting data generation");
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

    if args.clean {
        block_on(data_creator::clear_sqlite_data());
        println!("cleaned sqlite database");
    }

    // extract mysql data
    let table_1_data = block_on(data_querier::get_mysql_table_data(&args.table_name_1));
    let table_2_data = block_on(data_querier::get_mysql_table_data(&args.table_name_2));

    // build query statment
    let query_1 = format!("select * from {}", args.table_name_1);
    let query_2 = format!("select * from {}", args.table_name_2);

    let database_name = "test";
    let mysql_rows_1= block_on(data_querier::query_mysql(&query_1,database_name ));
    let mysql_rows_2 = block_on(data_querier::query_mysql(&query_2, database_name));
   
    let mut now = SystemTime::now();
    block_on(data_querier::mysql_table_to_sqlite_table(&mysql_rows_1, &table_1_data));
    match now.elapsed(){
        Ok(elapsed) => {
            println!("Time it took to migrate data to sqlite for table 1: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
        }
        Err(e) => {
            panic!("An error occured: {:?}", e);
        }
    }

    now = SystemTime::now();
    block_on(data_querier::mysql_table_to_sqlite_table(&mysql_rows_2, &table_2_data));
    match now.elapsed(){
        Ok(elapsed) => {
            println!("Time it took to migrate data to sqlite for table 2: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
        }
        Err(e) => {
            panic!("An error occured: {:?}", e);
        }
    }

    // compare the data
    now = SystemTime::now();
    let result = block_on(data_comparer::compare_sqlite_tables(&table_1_data,&table_2_data));
    match now.elapsed(){
        Ok(elapsed) => {
            println!("Time it took to compare both tables: {}.{}", elapsed.as_secs(),elapsed.subsec_millis());
        }
        Err(e) => {
            panic!("An error occured: {:?}", e);
        }
    }
    print_results(result);
}

fn print_results(result: data_comparer::ComparisonData){
    println!("rows in table 1 that are not in table 2: {}", result.unique_table_1_rows.len());
    println!("rows in table 2 that are not in table 1: {}", result.unique_table_2_rows.len());
    println!("rows that are different between the two tables: {}", result.changed_rows.len());
}
