use async_std::task::block_on;
use data_querier::TableData;

mod data_querier;
mod data_creator;
mod argument_parser;
mod data_comparer;

fn main() {
    let args = argument_parser::parse_arguments();

    if args.help {
        argument_parser::print_help();
    }

    if args.verbose {
        println!("verbose requested, please wait for this feautre to finish being implmented");
    }

    if args.version {
        println!("version requested, please wait for this feautre to finish being implmented");
    }

    // create new amount of random data
    if args.generate_data {
        block_on(data_creator::generate_mysql_test_data(args.number_of_rows_to_generate, &args.table_name_1, &args.table_name_2));
    }

    if args.clean {
        block_on(data_creator::clear_sqlite_data());
        println!("cleaned sqlite database");
    }

    // get the table data
    let table_1_data = block_on(data_querier::get_mysql_table_data(&args.table_name_1));
    let table_2_data = block_on(data_querier::get_mysql_table_data(&args.table_name_2));

    // select data we just created 
    let query_1 = format!("select * from {}", args.table_name_1);
    let query_2 = format!("select * from {}", args.table_name_2);
    let mysql_rows_1= block_on(data_querier::query_mysql(&query_1));
    let mysql_rows_2 = block_on(data_querier::query_mysql(&query_2));
    
    block_on(data_querier::mysql_to_sqlite(&mysql_rows_1, &table_1_data));
    block_on(data_querier::mysql_to_sqlite(&mysql_rows_2, &table_2_data));

    // compare the data
    let result = block_on(data_comparer::compare_sqlite_tables(&table_1_data,&table_2_data));
    if result {
        println!("tables are the same");
    }
    else {
        println!("tables are different");
    }
}

fn print_table_name(table_data: TableData) {
    println!("table name: {} has {} number of columns", table_data.table_name, table_data.columns.len());
}
