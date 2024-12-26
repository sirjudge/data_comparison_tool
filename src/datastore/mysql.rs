use crate::{
    interface::{
        config::Config,
        log::Log
    },
    models::table_data::TableData

};
use sqlx::{
    Row,
    Pool,
    mysql::{
        MySqlPoolOptions,
        MySql,
        MySqlRow,
    }
};

pub async fn drop_table(table_name: &str, log: &Log, config: &Config) {
    let pool = get_connection("ComparisonData", log, config).await;
    let drop_query = format!("drop table if exists {}", table_name);
    let result = sqlx::query(&drop_query).execute(&pool).await;
    match result {
        Ok(_) => {
            log.info(&format!("dropped table: {}", table_name));
        },
        Err(error) => {
            panic!("error occurred while dropping table: {:?}", error);
        },
    }
}

/// open a connection to the mysql databse, executes the query and then
/// returns a vector of the rows returned
pub async fn query(query_string: &str, database: &str, log: &Log, config: &Config) -> Vec<MySqlRow> {
    // open a connection to the test db and execute the query
    let pool = get_connection(database, log, config).await;
    let rows = sqlx::query(query_string).fetch_all(&pool).await;

    // if no errors return and rows isn't empty then return those rows, otherwise panic
    match rows {
        Ok(rows) => {
            if rows.is_empty() {
                panic!("no rows returned");
            }
            rows
        },
        Err(error) => {
            panic!("error: {:?}", error);
        },
    }
}

pub async fn get_connection(database_name: &str, log: &Log, config: &Config) -> Pool<MySql> {
    let connection_string =
        format!(
            "mysql://{}:{}@{}:{}/{}",
            config.database_1_config.db_user,
            config.database_1_config.db_password,
            config.database_1_config.db_host,
            config.database_1_config.db_port,
            "ComparisonData"
        );

    let result = MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&connection_string)
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
pub async fn get_table_data(table_name: &str, log: &Log, config: &Config) -> TableData {
    let pool = get_connection("ComparisonData", log, config).await;
    //BUG: This isn't getting populated for some reason

    let select_query = format!("select * from {} limit 1", table_name);

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
            log.error(&format!("executing query: {}", select_query));
            panic!("error occurred while fetching table data from {:?}", error);
        },
    }
}

