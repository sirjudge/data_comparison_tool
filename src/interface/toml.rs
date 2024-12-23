use toml;
use serde::Deserialize;

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
    pub number_of_rows_to_generate: u32
}

#[derive(Deserialize)]
pub struct ComparisonOptions{
    pub create_sqlite_comparison_files: bool,
    pub database_1_config: DatabaseConfig1,
    pub database_2_config: DatabaseConfig1,
    pub in_memory_sqlite: bool,
    pub clean: bool
}

#[derive(Deserialize)]
pub struct LogConfig {
    pub log_file: String,
    pub output_file_name: String,
    pub output_file_type: OutputFileType,
    pub log_level: String,
    pub verbose: bool,
    pub help: bool,
    pub auto_yes: bool,

    // consider moving versioning to it's own struct
    pub version: bool
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

pub struct Interface {
    pub tui: bool,
}

impl Config {
    pub fn new (toml_file_path: &str) -> Config {
        let toml_str = std::fs::read_to_string(toml_file_path).unwrap();
        let args: Config =
            toml::from_str(&toml_str).unwrap();
        args
    }
}
