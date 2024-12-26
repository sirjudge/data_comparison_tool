use data_comparison_tool::{
    datastore::{
        generator,
        mysql
    },
    interface::{
        config::{
            Config,
            DatabaseConfig
        },
        log,
        log_options::{LogOutput, LogVerbosity}
    }
};
use async_std::task::block_on;

// todo: should have a method here that generates the data for the tables on start up and then does
// the same on tear down
pub fn setup() -> (Config, log::Log) {
    let config = Config::new("tests/comp.toml");
    let mut log = log::Log::new(&config);
    log.set_verbose(LogVerbosity::Debug);
    log.set_output_type(LogOutput::Console);
    (config, log)
}

/// responsible for cleaning up test runtime artifacts
pub fn teardown() {
    panic!("teardown not implemented");
}

/// generates a test table in mysql
pub async fn generate_mysql_table(config:&Config, table_name: &str, db_config: &DatabaseConfig) -> usize {
    let (_, log) = self::setup();

    block_on(data_comparison_tool::datastore::sqlite::drop_table(table_name, &log));
    block_on(generator::generate_table(config, &log, db_config));

    let select_query = format!("select * from {}", table_name);
    let mysql_pool = mysql::get_connection(&log, &config.database_1_config).await;

    let result = sqlx::query(&select_query).fetch_all(&mysql_pool).await;
    match result {
        Ok(rows) => {
            rows.len()
        },
        Err(error) => {
            panic!("error occurred while fetching rows from mysql table: {:?}", error);
        },
    }
}
