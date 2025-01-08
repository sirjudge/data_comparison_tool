use data_comparison_tool::interface::config;

#[test]
pub fn default_config(){
    let toml_config = config::Config::default();
    // Assert we have the correct values in test/comp.toml
    // log related information
    assert!(toml_config.log_config.log_level == "DEBUG");
    assert!(toml_config.log_config.log_file == "test.log");

    // db config
    assert!(toml_config.database_1_config.db_name.is_empty());
    assert!(toml_config.database_1_config.db_port == 3306);
    assert!(toml_config.database_1_config.db_password.is_empty());
    assert!(toml_config.database_1_config.db_user.is_empty());
    assert!(toml_config.database_1_config.db_host.is_empty());
}

#[test]
pub fn comparison_default_toml_file_parse_ok(){
    let toml_config = config::Config::new_from_file("src/comparison_default.toml");
    assert!(toml_config.log_config.log_level == "DEBUG");
}
