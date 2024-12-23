use data_comparison_tool::interface::config;

/// toml file
#[test]
pub fn get_args_from_toml(){
    // get the args from test file into new config struct
    let toml_config = config::Config::new("tests/comp.toml");

    // Assert we have the correct values in test/comp.toml
    // log related information
    assert!(toml_config.log_config.log_level == "DEBUG");
    assert!(toml_config.log_config.log_file == "test.log");

    // db config
    assert!(toml_config.comparison_options.database_1_config.db_name == "test_db");
    assert!(toml_config.comparison_options.database_1_config.db_port == 1234);
    assert!(toml_config.comparison_options.database_1_config.db_password == "testPassword");
    assert!(toml_config.comparison_options.database_1_config.db_user == "testUser");
    assert!(toml_config.comparison_options.database_1_config.db_host == "localhost");
}


/*
[LogConfig]
log_level = "DEBUG"
log_file = "test.log"

[DatabseConfig]
db_name = "test_db"
db_port = 1234
db_password = "testPassword"
db_user = "testUser"
db_host = "localhost"
 */
