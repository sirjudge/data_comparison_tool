use chrono::Local;
use std::process::exit;

pub enum OutputFileType {
    Csv,
    Json
}

/// Struct to hold the arguments passed in from the command line
pub struct Arguments {
    /// run with terminalUi
    pub tui: bool,

    /// output file path to export data to
    pub output_file_name: String,

    pub output_file_type: OutputFileType,

    /// MySql query to run to generate the first tabl
    pub mysql_query_1: String,

    /// MySql query to run to generate the second table
    pub mysql_query_2: String,

    /// flag to generate new data in MySql
    pub generate_data: bool,

    /// flag to enable verbose output
    pub verbose: bool,

    /// flag to print version information
    pub version: bool,

    /// flag to print help information
    pub help: bool,

    /// flag to clean the sqlite database files up
    pub clean: bool,

    /// number of rows to generate in mysql. Requires the -gen flag to be set
    pub number_of_rows_to_generate: i32,

    /// name of the first table to compare
    pub table_name_1: String,

    /// name of the second table to compare
    pub table_name_2: String,

    /// flag to create sqlite comparison files while in flight
    pub create_sqlite_comparison_files: bool,

    /// flag to use in memory sqlite database instead of file based
    pub in_memory_sqlite: bool,

    /// flag to auto select yes to all prompts
    pub auto_yes: bool
}

/// prints the urrent flags and their descriptions
pub(crate) fn print_help(){
    println!("Help requested! This is a tool to help compare large data sets between mysql and sqlite");
    println!("Usage: data_comparison");
    println!("\t-h : print this help message");
    println!("\t-help : print this help message");
    println!("\t-tui : run with terminal ui");
    println!("\t-q1=<query> : specify a first mysql query to run");
    println!("\t-q2=<query> : specify a second mysql query to run");
    println!("\t-gen : generate new data in mysql");
    println!("\t-verbose : verbose output");
    println!("\t-version : print version information");
    println!("\t-c : clean sqlite database");
    println!("\t-t1=<table_name> : specify the name of the first table to compare");
    println!("\t-t2=<table_name> : specify the name of the second table to compare");
    println!("\t-in-memory : use an in memory sqlite database instead of file based");
    println!("\t-create-in-flight : create sqlite comparison files while in flight");
    println!("\t-auto-yes : automatically answer yes to all prompts");
    println!("\t-output=<output_file> : specify the name of the output csv file");
}

impl Arguments {
    pub fn new () -> Arguments {
        // init argument struct
        let current_date_stamp = Local::now().format("%Y%m%d%H%M%S").to_string();

        // declare default arguments
        let mut return_arguments = Arguments{
            mysql_query_1: "".to_string(),
            mysql_query_2: "".to_string(),
            tui: false,
            generate_data: false,
            verbose: false,
            version: false,
            help: false,
            clean: true,
            number_of_rows_to_generate: 0,
            table_name_1: format!("table_1{}", current_date_stamp),
            table_name_2: format!("table_2{}", current_date_stamp),
            create_sqlite_comparison_files: true,
            in_memory_sqlite: false,
            auto_yes: false,
            output_file_name: "".to_string(),
            output_file_type: OutputFileType::Csv
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
                    "-output" => {
                        println!("output file: {}", value.unwrap());
                        return_arguments.output_file_name = value.unwrap().to_string();
                        let output_file = return_arguments.output_file_name.clone();
                        let file_type = output_file.split('.');
                        let file_type = file_type.last().unwrap();
                        match file_type {
                            "csv" => {
                                return_arguments.output_file_type = OutputFileType::Csv;
                            }
                            "json" => {
                                return_arguments.output_file_type = OutputFileType::Json;
                            }
                            _ => {
                                println!("Unknown file type: {}", file_type);
                            }
                        }
                    }
                    &_ => {
                        println!("Unknown argument:{}",arg);
                    }
                }
            }
            // if arg doesn't contain an = then we can just check the whole flag
            else {
                match arg.as_str() {
                    "-tui" => {
                        return_arguments.tui = true;
                    }
                    "-help" => {
                        return_arguments.help = true;
                        print_help();
                        exit(0);
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
                    "-c" => {
                        return_arguments.clean = true;
                        println!("cleaning sqlite database files");
                    }
                    "-g" => {
                        return_arguments.generate_data = true;
                        println!("generating data");
                    }
                    "-in-memory" => {
                        return_arguments.in_memory_sqlite = true;
                        println!("using in memory sqlite database");
                    }
                    "-create-in-flight" => {
                        return_arguments.create_sqlite_comparison_files = true;
                        println!("creating sqlite comparison files while in flight");
                    }
                    "-auto-yes" => {
                        return_arguments.auto_yes = true;
                        println!("auto yes enabled");
                    }
                    &_ => {
                        println!("Unknown argument:{}",arg);
                    }
                }
            }
        }

        // finally retured the parsed arugments
        return_arguments
    }
}

