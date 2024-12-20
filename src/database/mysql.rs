use crate::{
    interface::log::Log,
    models::table_data::TableData
};
use sqlx::{
    Row,
    Pool,
    mysql::{
        MySqlPoolOptions,
        MySql
    }
};
use std::env;


/// open a connection to the mysql databse
pub(crate) async fn get_mysql_connection(database_name: &str, log: &Log) -> Pool<MySql> {
    let database_name_override = "ComparisonData";
    // BUG: the connection string is definitely an env variable but is not being populated
    // correctly.
    log.debug("attempting to get mysql connection string from env var");
    log.debug(&format!("env vars: {:?}", env::vars()));

    let connection_string_env_var = env::var("MYSQL_CONNECTION_STRING_USER");
    let mysql_connection_string = match connection_string_env_var {
        Ok(connection_string_env_var) => connection_string_env_var,
        Err(_) => {
            log.warn("MYSQL_CONNECTION_STRING_USER not set, using default connection string");
            format!(
                "mysql://nico:RealPassw0rd@localhost:3306/{}",
                database_name_override
            )
        },
    };

    // attempt to connect and handle success/fail accordingly
    let result = MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&mysql_connection_string)
        .await;

    match result {
        Ok(pool) => {
            log.info(&format!("connected to mysql database: {}", database_name));
            pool
        },
        Err(error) => {
            panic!("unable to connect to mysql db {}", error);
        },
    }
}

/// given a table now select 1 row from the table and extract
/// a list of columns and the primary key
pub(crate) async fn get_mysql_table_data(table_name: &str, log: &Log) -> TableData {
    let pool = get_mysql_connection("ComparisonData", log).await;
    let select_query = format!("select * from {} limit 1", table_name);

    //BUG: when using `cargo test` this query is failing to look up the table
    //for some reason
    let result = sqlx::query(&select_query).fetch_one(&pool).await;
    match result {
        Ok(row) => {
            let columns = row.columns();
            let mut column_names = Vec::new();
            for column in columns {
                column_names.push(column.clone());
            }

            //TODO: add support to extract the actual primary key
            TableData {
                table_name: table_name.to_string(),
                columns: column_names,
                primary_key: "id".to_string(),
            }
        },
        Err(error) => {
            panic!("error occurred while fetching table data from {:?}", error);
        },
    }
}

