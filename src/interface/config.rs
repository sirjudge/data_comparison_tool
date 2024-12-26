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
    pub database_1_config: DatabaseConfig,
    pub database_2_config: DatabaseConfig,
    pub data_generation: DataGeneration,
    pub tui: bool,
    pub version: bool
}

#[derive(Deserialize)]
pub struct ComparisonOptions{
    pub output_file_name: String,
    pub output_file_type: OutputFileType,
    pub create_sqlite_comparison_files: bool,
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
pub struct DatabaseConfig {
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
pub enum OutputFileType {
    Csv,
    Json
}

#[derive(Deserialize)]
pub struct DataGeneration {
    pub generate_data: bool,
    pub number_of_rows_to_generate: i32,
}

impl Default for Config {
    fn default() -> Self {
        let now = Local::now();
        Config {
            data_generation: DataGeneration {
                generate_data: false,
                number_of_rows_to_generate: 0,
            },
            comparison_options: ComparisonOptions {
                output_file_name: format!("comparison_output_{}.csv", now.format("%Y%m%d%H%M%S")),
                output_file_type: OutputFileType::Csv,
                create_sqlite_comparison_files: true,
                in_memory_sqlite: true,
                clean: true
            },
            log_config: LogConfig {
                //log_file: format!("data_comparison_{}.log", now.format("%Y%m%d%H%M%S")),
                log_file:"test.log".to_string(),
                log_level: "DEBUG".to_string(),
                verbose: true,
                help: false,
                auto_yes: false,
                log_output_type: LogOutput::File,
            },
            tui: false,
            version: false,
            database_1_config: DatabaseConfig {
                db_name: String::from(""),
                db_user: String::from(""),
                db_password: String::from(""),
                db_host: String::from(""),
                db_port: 3306,
                table_name: String::from(""),
                query: String::from(""),
                database_type: DatabaseType::MySql
            },
            database_2_config: DatabaseConfig {
                db_name: String::from(""),
                db_user: String::from(""),
                db_password: String::from(""),
                db_host: String::from(""),
                db_port: 3306,
                table_name: String::from(""),
                query: String::from(""),
                database_type: DatabaseType::MySql
            },
        }
    }
}

impl Config {
    pub fn new (toml_file_path: &str) -> Config {
        let toml_str = std::fs::read_to_string(toml_file_path).unwrap();
        let config: Config =
            match toml::from_str(&toml_str) {
                Ok(config) => config,
                Err(e) => {
                    println!("Error parsing toml file:{} with error:{}", toml_file_path ,e);
                    panic!("Uh oh spaghetti-o's");
                }
            };

        config
    }
}
