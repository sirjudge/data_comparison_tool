use data_comparison_tool::{
    datastore::{
        generator, mysql
    },
    interface::{
        config,
        log,
        log_options::{LogOutput, LogVerbosity}
    }
};
use async_std::task::block_on;

// todo: should have a method here that generates the data for the tables on start up and then does
// the same on tear down
pub fn setup() -> (config::Config, log::Log) {
    let config = config::Config::new("tests/comp.toml");
    let mut log = log::Log::new(&config);
    log.set_verbose(LogVerbosity::Debug);
    log.set_output_type(LogOutput::Console);
    (config, log)
}

/// responsible for cleaning up test runtime artifacts
pub fn teardown() {
    panic!("teardown not implemented");
}

pub fn drop_comparison_tables(table_name: &str, log: &log::Log, config:&config::Config) {
    log.info(&format!("dropping tables: {}", table_name));
    block_on(mysql::drop_table(table_name, log, config));

    let changed_row_table = format!("changedRows_{}", table_name);
    log.info(&format!("dropping tables: {}", changed_row_table));
    block_on(mysql::drop_table(&changed_row_table, log, config));

    let unique1 = format!("unique_{}", table_name);
    log.info(&format!("dropping tables: {}", unique1));
    block_on(mysql::drop_table(&unique1, log, config));
}

/// generates a test table in mysql
pub async fn generate_mysql_table(table_name: &str, config: &config::Config) -> usize {
    let (_, log) = self::setup();

    block_on(data_comparison_tool::datastore::sqlite::drop_table(table_name, &log));
    block_on(generator::create_new_mysql_table_data(20, table_name, &log, config));

    let select_query = format!("select * from {}", table_name);
    let mysql_pool = mysql::get_connection(&log, config).await;
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
