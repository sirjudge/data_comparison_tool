use chrono::{Local};

pub struct Arguments {
    pub mysql_query_1: String,
    pub mysql_query_2: String,
    pub generate_data: bool,
    pub verbose: bool,
    pub version: bool,
    pub help: bool,
    pub clean: bool,
    pub number_of_rows_to_generate: i32,
    pub table_name_1: String,
    pub table_name_2: String
}

pub(crate) fn print_help(){
    println!("Help requested! This is a tool to help compare large data sets between mysql and sqlite"); 
    println!("Usage: data_comparison");
    println!("\t-h : print this help message");
    println!("\t-help : print this help message");
    println!("\t-q1=<query> : specify a first mysql query to run");
    println!("\t-q2=<query> : specify a second mysql query to run");
    println!("\t-gen : generate new data in mysql");
    println!("\t-verbose : verbose output");
    println!("\t-version : print version information");
    println!("\t-c : clean sqlite database");
    println!("\t-t1=<table_name> : specify the name of the first table to compare");
    println!("\t-t2=<table_name> : specify the name of the second table to compare");
}

pub(crate) fn parse_arguments() -> Arguments{
    // init argument struct
    let current_date_stamp = Local::now().format("%Y%m%d%H%M%S").to_string();

    let mut return_arguments = Arguments{
        mysql_query_1: "select * from table_1".to_string(),
        mysql_query_2: "select * from table_2".to_string(),
        generate_data: false,
        verbose: false,
        version: false,
        help: false,
        clean: true,
        number_of_rows_to_generate: 0,
        table_name_1: format!("table_1{}", current_date_stamp),
        table_name_2: format!("table_2{}", current_date_stamp)
    };
   
    // loop over each argument
    // first argument is the name of the program so we can ignore it
    for arg in std::env::args().skip(1){
        // if arg contains '=' split the flag + value and print
        if arg.contains('='){
            let mut split_string = arg.split('=');
            let flag = split_string.next();
            let value = split_string.next();
            std::env::set_var(flag.unwrap(), value.unwrap());
            match flag.unwrap() {
                "-q" => {
                    return_arguments.mysql_query_1= value.unwrap().to_string();
                    println!("query: {}", return_arguments.mysql_query_1);
                }
                "-gen" => {
                    return_arguments.generate_data = true;
                    let number_of_rows = value.unwrap().parse::<i32>().unwrap();
                    return_arguments.number_of_rows_to_generate = number_of_rows;
                }
                "-t1" => {
                    let table_name = value.unwrap();
                    return_arguments.table_name_1 = format!("{}{}",table_name, current_date_stamp);
                    println!("table name 1: {}", return_arguments.table_name_1);
                }
                "-t2" => {
                    let table_name = value.unwrap();
                    return_arguments.table_name_2 = format!("{}{}",table_name, current_date_stamp);
                    println!("table name 2: {}", return_arguments.table_name_2);
                }
                &_ => {
                    println!("Unknown argument:{}",arg);
                }
            }
        }
        // if arg doesn't contain an = then we can just check the whole flag
        else {
            match arg.as_str() {
                "-c" => {
                    return_arguments.clean = true;
                    println!("cleaning sqlite database files");
                }
                "-help" => {
                    return_arguments.help = true;
                    print_help();
                }
                "-h" => {
                    return_arguments.help = true;
                    print_help();
                }
                "-version" => {
                    return_arguments.version = true;
                    const VERSION: &str = env!("CARGO_PKG_VERSION");
                    println!("version: {}", VERSION);
                }
                "-verbose" => {
                    return_arguments.verbose = true;
                    println!("verbose output enabled");
                }
                "-g" => {
                    return_arguments.generate_data = true;
                    println!("generating data");
                }
                &_ => {
                    println!("Unknown argument:{}",arg);
                }
            }
        }
    }
    return_arguments
}
