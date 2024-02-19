use async_std::task::block_on;

mod data;
mod data_unit_test;
mod argument_parser;


fn main() {
    let args = argument_parser::parse_arguments();

    if args.help {
        println!("help requested, please wait for this feautre to finish being implmented");
    }

    if args.verbose {
        println!("verbose requested, please wait for this feautre to finish being implmented");
    }

    if args.version {
        println!("version requested, please wait for this feautre to finish being implmented");
    }

    // create new amount of random data
    if args.generate_data {
        block_on(data_unit_test::create_new_data(1000));
        println!("created 100 rows in mysql");
    }

    // test sqlite connection
    let query = "select 1 as number";
    let sqlite_row = block_on(data::query_sqlite(query));
    println!(" returned # of sqlite rows: {:?}",sqlite_row.len());



    // select data we just created 
    let query = "select * from test_table";
    let mysql_rows = block_on(data::query_mysql(query));
    block_on(data::mysql_to_sqlite(mysql_rows));

}

