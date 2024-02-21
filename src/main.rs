use async_std::task::block_on;

mod data;
mod data_creator;
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
        block_on(data_creator::create_new_data(1000));
        println!("created 100 rows in mysql");
    }

    // select data we just created 
    // TODO: pass in query parameter from command line args
    let query = "select * from test_table";
    let mysql_rows = block_on(data::query_mysql(query));
    block_on(data::mysql_to_sqlite(&mysql_rows));
}

#[cfg(test)]
mod tests {
    use crate::data_creator;
    
    //TODO: documentation states this is important
    // figure out why and consider if keeping is necessary but feels weird
    // in my bones
    // nico - 2/20/2024
    use super::*;

    #[test]
    fn sqlite_constructor() {
        block_on(data_creator::create_new_data(1000));
    }

    #[test]
    fn sqlite_destroyer(){
        block_on(data_creator::clear_sqlite_data()); 
    }
}

