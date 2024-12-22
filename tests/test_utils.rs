use data_comparison_tool::{
    interface::{
    argument_parser,
    log,
    log_options::LogVerbosity
    },
    datastore::{
        mysql,
        generator,
        sqlite
    }
};
use async_std::task::block_on;

// todo: should have a method here that generates the data for the tables on start up and then does
// the same on tear down
pub fn setup() -> (argument_parser::Arguments, log::Log) {
    let mut arguments = argument_parser::Arguments::new();
    arguments.tui = false;
    arguments.help = false;
    arguments.table_name_1 = "a_testTable1".to_string();
    arguments.table_name_2 = "a_testTable2".to_string();
    arguments.generate_data = true;
    arguments.clean = false;
    arguments.verbose = true;
    arguments.number_of_rows_to_generate = 20;
    let mut log = log::Log::new(&arguments);
    log.set_verbose(LogVerbosity::Debug);

    (arguments, log)
}

/// responsible for cleaning up test runtime artifacts
pub fn teardown() {
    panic!("teardown not implemented");
}


/// generates a test table in mysql
pub async fn generate_mysql_table(table_name: &str) -> usize {
    let (_, log) = self::setup();

    block_on(data_comparison_tool::datastore::sqlite::drop_table(table_name, &log));
    block_on(generator::create_new_mysql_table_data(20, table_name, &log));

    let select_query = format!("select * from {}", table_name);
    let mysql_pool = mysql::get_connection("ComparisonData",&log).await;
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
