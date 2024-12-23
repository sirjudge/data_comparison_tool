use data_comparison_tool::interface::argument_parser::Arguments;
use data_comparison_tool::interface::toml;

/// Test to make sure non user friendly settings are turned off or set to sane
/// defaults
#[test]
pub fn get_default_args(){
    let args = Arguments::default();

    // default should always be no tui
    assert!(!args.tui);
}

/// test to make sure the aguments are being correctly parsed from a
/// toml file
#[test]
pub fn get_args_from_toml(){
    // get the args from test file into new config struct
    let toml_config = toml::config::new("tests/comp.toml");

    // Assert we have the correct values in test/comp.toml
    // files
    assert!(toml_config.log_config.log_level == "DEBUG");
    assert!(toml_config.log_config.log_file == "test.log");

    assert!(toml_config.database_config.db_name == "test_db");
    assert!(toml_config.database_config.db_port == 1234);
    assert!(toml_config.database_config.db_password == "testPassword");
    assert!(toml_config.database_config.db_user == "testUser");
    assert!(toml_config.database_config.db_host == "localhost");
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
