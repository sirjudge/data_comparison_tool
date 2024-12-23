use toml;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct config {
    pub database_config: database_config,
    pub log_config: log_config
}

#[derive(Deserialize)]
pub struct log_config {
    pub log_level: String,
    pub log_file: String,
}

#[derive(Deserialize)]
pub struct database_config {
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
}

impl config {
    pub fn new (toml_file_path: &str) -> config {
        let toml_str = std::fs::read_to_string(toml_file_path).unwrap();
        let args: config =
            toml::from_str(&toml_str).unwrap();
        args
    }
}
