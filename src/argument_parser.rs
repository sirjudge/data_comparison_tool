pub struct Arguments {
    pub mysql_query: String, 
    pub generate_data: bool,
    pub verbose: bool,
    pub version: bool,
    pub help: bool,
    pub clean: bool,
    pub number_of_rows_to_generate: i16,
    pub table_name_1: String,
    pub table_name_2: String
}

pub(crate) fn print_help(){
    println!("Help requested! This is a tool to help compare large data sets between mysql and sqlite"); 
    println!("Usage: ");
    println!("\t-q=<query> : specify a mysql query to run");
    println!("\t-gen : generate new data in mysql");
    println!("\t-v : verbose output");
    println!("\t-h : print this help message");
    println!("\t-d : clean sqlite database");
    println!("\t--version : print version information");
    println!("\t-t1=<table_name> : specify the name of the first table to compare");
    println!("\t-t2=<table_name> : specify the name of the second table to compare");
}

pub(crate) fn parse_arguments() -> Arguments{
    // init argument struct
    let mut return_arguments = Arguments{
        mysql_query: "select * from test_table".to_string(),
        generate_data: false,
        verbose: false,
        version: false,
        help: false,
        clean: true,
        number_of_rows_to_generate: 0,
        table_name_1: "test_table_1".to_string(),
        table_name_2: "test_table_2".to_string()
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
                    return_arguments.mysql_query = value.unwrap().to_string();
                }
                "-gen" => {
                    return_arguments.generate_data = true;
                    let number_of_rows = value.unwrap().parse::<i16>().unwrap();
                    return_arguments.number_of_rows_to_generate = number_of_rows;
                }
                "-t1" => {
                    return_arguments.table_name_1 = value.unwrap().to_string();
                }
                "-t2" => {
                    return_arguments.table_name_2 = value.unwrap().to_string();
                }
                &_ => {
                    println!("Unknown argument:{}",arg);
                }
            }
        }
        // if arg doesn't contain an = then we can just check the whole flag
        else {
            match arg.as_str() {
                "-d" => {
                    return_arguments.clean = true;
                }
                "-h" => {
                    return_arguments.help = true;
                }
                "-v" => {
                    return_arguments.verbose = true;
                }
                "-g" => {
                    return_arguments.generate_data = true;
                }
                &_ => {
                    println!("Unknown argument:{}",arg);
                }
            }
        }
    }
    return return_arguments;
}
