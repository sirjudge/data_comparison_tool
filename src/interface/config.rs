use toml;
use serde::Deserialize;
use crate::interface::log_options::LogOutput;
 use chrono::Local;

#[derive(Deserialize)]
pub enum DatabaseType {
    MySql,
    Sqlite
}

#[derive(Deserialize)]
pub struct Config {
    pub comparison_options: ComparisonOptions,
    pub log_config: LogConfig,
    pub database_1_config: DatabaseConfig,
    pub database_2_config: DatabaseConfig,
    pub data_generation: DataGeneration,
    pub globals: Globals
}

#[derive(Deserialize)]
pub struct Globals {
    pub tui: bool,
    pub version: bool,
    pub config_file_path: String
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
    pub clean: bool
}

impl Default for Config {
    fn default() -> Self {
        let now = Local::now();
        Config {
            globals: Globals {
                tui: false,
                version: false,
                config_file_path: "src/comparison_default.toml".to_string()
            },
            data_generation: DataGeneration {
                generate_data: false,
                number_of_rows_to_generate: 0,
                clean: false,
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
    pub fn new() -> Config {
        // if we have -config flag, parse the config file first before
        // applying cli arguments
        let args: Vec<String>= std::env::args().collect();
        if args.contains(&"-config".to_string()) {
            let config_index = args.iter().position(|x| x == "-config").unwrap();
            let config_file = &args[config_index + 1];
            Config::from_config_file(config_file);
        }

        Config::from_arguments()
    }

    pub fn from_config_file(config_file_path: &str) -> Config {
        // load config file into a a string
        match std::fs::read_to_string(config_file_path){


            Ok(config_string) => {
                // if we successfully read into a string parse the toml
                let verbose = true;
                if verbose {
                    println!("successfully read config file:{}", config_file_path);
                    println!("config_string:{}", config_string);
                }
                let config: Config =
                    match toml::from_str(&config_string) {
                        Ok(config) => config,
                        Err(e) => {
                            panic!("Error parsing toml file:{} with error:{}", config_file_path ,e);
                        }
                    };

                if !Self::validate_config(&config).is_empty() {
                    panic!("Invalid configuration detected");
                }
                config
            },
            Err(e) => {
                panic!("Error reading config file:{} with error:{}", config_file_path ,e);
            }
        }

    }

    /// if args is empty or only contains the program name return the
    /// default config otherwise parse the arguments and return the default
    /// config with the arguments applied. If the -config flag is passed in it
    /// will parse the config file first and then overwrite the file options
    /// with any CLI flags passed in
    pub fn from_arguments() -> Config {
        println!("Parsing CLI arguments");
        let mut config: Config = Config::default();
        let args: Vec<String> = std::env::args().collect();

        // if args is empty return default config
        if args.is_empty() {
            return config;
        }

        // loop through each flag and apply the key value paris
        for arg in args.iter() {
            // handle key/value pairs if = is present in the flag
            if arg.contains("=") {
                let arg_parts: Vec<&str> = arg.split("=").collect();
                let key = arg_parts[0];
                let value = arg_parts[1];
                match key {
                    "-t1" => config.database_1_config.table_name = value.to_string(),
                    "-t2" => config.database_2_config.table_name = value.to_string(),
                    "-output" => config.comparison_options.output_file_name = value.to_string(),
                    "-gen" => {
                        config.data_generation.generate_data = true;
                        config.data_generation.number_of_rows_to_generate = value.parse().unwrap();
                    },
                    _ => {}
                }
            }
            // handle flags without values that act as true/false if they
            // exist or don't
            else {
                match arg.as_str() {
                    "-h" | "-help" => {
                        config.log_config.help = true;
                        return config;
                    },
                    "-tui" => {
                        config.globals.tui = true;
                        return config;
                    },
                    "-gen" => {
                        config.data_generation.generate_data = true;
                        return config;
                    },
                    "-verbose" => {
                        config.log_config.verbose = true;
                    },
                    "-version" => {
                        config.globals.version = true;
                        return config;
                    },
                    "-c" => {
                        config.comparison_options.clean = true;
                    },
                    "-in-memory" => {
                        config.comparison_options.in_memory_sqlite = true;
                    },
                    "-create-in-flight" => {
                        config.comparison_options.create_sqlite_comparison_files = true;
                    },
                    "-auto-yes" => {
                        config.log_config.auto_yes = true;
                    },
                    _ => {}
                }
            }
        }

        let errors = Self::validate_config(&config);
        if !errors.is_empty() {
            config
        }
        else {
            panic!("Invalid configuration detected");
        }
    }

    pub fn validate_db_config(config: &DatabaseConfig) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();

        if config.db_name.is_empty()
        {
            errors.push("empty database name".to_string());
        }

        if config.db_user.is_empty()
        {
            errors.push("empty database user".to_string());
        }

        if config.db_password.is_empty()
        {
            errors.push("empty database password".to_string());
        }

        if config.db_host.is_empty()
        {
            errors.push("empty database host".to_string());
        }

        if config.table_name.is_empty()
        {
            errors.push("empty table name".to_string());
        }

        errors
    }

    pub fn validate_config(config: &Config) -> Vec<String> {
        // initialize empty list of strings for
        // any errors that happen during validation
        let mut errors: Vec<String> = Vec::new();

        println!();
        println!("{}BEGIN VALIDATION{}", "=".repeat(10), "=".repeat(10));
        println!();
        // database connection validation
        let db_1_validation = &mut Self::validate_db_config(&config.database_1_config);
        if !db_1_validation.is_empty() {
            println!("db_1_validation: {:?}", db_1_validation);
            println!();
            errors.append(db_1_validation);
        }
        let db_2_validation = &mut Self::validate_db_config(&config.database_1_config);
        if !db_2_validation.is_empty() {
            println!("db_2_validation: {:?}", db_2_validation);
            println!();
            errors.append(db_2_validation);
        }

        if errors.is_empty() {
            println!("Configuration is valid");
        }
        else {
            println!("Configuration is invalid: {:?}", errors);
        }
        // print any errors that occur during runtime
        println!();
        println!("{}END VALIDATION{}", "=".repeat(10), "=".repeat(10));

        errors
    }
}
