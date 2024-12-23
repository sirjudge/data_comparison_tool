use toml;
use serde::Deserialize;
use crate::interface::log_options::LogOutput;
 use chrono::Local;

#[derive(Deserialize)]
pub enum DatabaseType {
    MySql,
    Sqlite
}

//TODO: Starting names off a bit heavy handed with longer
//names, think about cutting down later or just calling the slightly
//more descriptive names acceptable
#[derive(Deserialize)]
pub struct Config {
    pub comparison_options: ComparisonOptions,
    pub log_config: LogConfig,
    pub generate_data: bool,
    pub number_of_rows_to_generate: i32,
    pub tui: bool,
    pub version: bool
}

#[derive(Deserialize)]
pub struct ComparisonOptions{
    pub output_file_name: String,
    pub output_file_type: OutputFileType,
    pub create_sqlite_comparison_files: bool,
    pub database_1_config: DatabaseConfig1,
    pub database_2_config: DatabaseConfig1,
    pub in_memory_sqlite: bool,
    pub clean: bool
}

#[derive(Deserialize)]
pub struct LogConfig {
    pub log_file: String,
    pub log_level: String,
    pub verbose: bool,
    pub help: bool,
    pub auto_yes: bool,
    pub log_output_type: LogOutput,
}

// TODO: The following double init is hacky
// but it works. Might not be worth too much more hassle
// but should definitely be re-visited if optimization
// is required
#[derive(Deserialize)]
pub struct DatabaseConfig1{
    // connection level
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,

    // data extraction level
    pub table_name: String,
    pub query: String,
    pub database_type: DatabaseType,
}

#[derive(Deserialize)]
pub struct DatabaseConfig2 {
    // connection level
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,

    // data extraction level
    pub table_name: String,
    pub query: String,
    pub query_type: DatabaseType,
}

#[derive(Deserialize)]
pub enum OutputFileType {
    Csv,
    Json
}


impl Default for Config {
    fn default() -> Self {
        Config {
            comparison_options: ComparisonOptions {
                output_file_name: String::from(""),
                output_file_type: OutputFileType::Csv,
                create_sqlite_comparison_files: false,
                database_1_config: DatabaseConfig1 {
                    db_name: String::from(""),
                    db_user: String::from(""),
                    db_password: String::from(""),
                    db_host: String::from(""),
                    db_port: 0,
                    table_name: String::from(""),
                    query: String::from(""),
                    database_type: DatabaseType::MySql
                },
                database_2_config: DatabaseConfig1 {
                    db_name: String::from(""),
                    db_user: String::from(""),
                    db_password: String::from(""),
                    db_host: String::from(""),
                    db_port: 0,
                    table_name: String::from(""),
                    query: String::from(""),
                    database_type: DatabaseType::MySql
                },
                in_memory_sqlite: false,
                clean: false
            },
            log_config: LogConfig {
                log_file: String::from(""),
                log_level: String::from(""),
                verbose: false,
                help: false,
                auto_yes: false,
                log_output_type: LogOutput::File,
            },
            generate_data: false,
            number_of_rows_to_generate: 0,
            tui: false,
            version: false
        }
    }
}

impl Config {
    pub fn new (toml_file_path: &str) -> Config {
        let toml_str = std::fs::read_to_string(toml_file_path).unwrap();
        let config: Config =
            toml::from_str(&toml_str).unwrap();
        config
    }

    pub fn new_from_args() -> Config {
        let mut config = Config::default();
        // let current_date_stamp = Local::now().format("%Y%m%d%H%M%S").to_string();

        if std::env::args().len() == 1 {
            println!("No args passed in, running with default args");
            return config
        }

        config
    }

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
}
