use async_std::task::block_on;
use rand::{ thread_rng, Rng};
use std::time::SystemTime;
use crate::{
    datastore::{
        mysql::get_connection,
        transformer::mysql_type_to_sqlite_type
    },
    interface::{
        config::{self, DatabaseConfig}, log::Log
    },
};
use sqlx::{
    Pool,
    mysql::MySqlRow,
    Row,
    Column,
    TypeInfo
};

pub async fn generate_table(config:&config::Config, log:&Log, db_config: &DatabaseConfig, profile: bool){
    log.debug("data creation underway");

    let pool = get_connection(log, db_config).await;
    let create_new_table_query = format!(
        "CREATE TABLE IF NOT EXISTS {}
        (
            id INT NOT NULL AUTO_INCREMENT,
            randomNumber INT NOT NULL,
            secondRandomNumber INT NOT NULL,
            randomString VARCHAR(255) NOT NULL,
            secondRandomString VARCHAR(255) NOT NULL,
            PRIMARY KEY (id)
        )", db_config.table_name);

    let result = sqlx::query(&create_new_table_query)
        .execute(&pool)
        .await;
    match result {
        Ok(_) => {
            log.debug(&format!("created table: {}", db_config.table_name));
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }


    //OPTIMIZE: this is a really painful thing to see, we should be able to
    //do this a bit faster than making one huge string as an insert statement
    //and then executing it. Consider maybe doing this asyncronously
    //and/or in parallel since order doesn't matter with random data creation

    //TODO: Similar sentement as above
    // todo: can speed this up by using prepared statement I think and passing data in via
    // parameterized query
    let mut insert_query =
        format!(
            "INSERT INTO {}
            (randomNumber,secondRandomNumber,randomString,secondRandomString)
            VALUES ", db_config.table_name
        );

    for _i in 0..config.data_generation.number_of_rows_to_generate {
        insert_query.push_str(
            &format!(
                "({},'{}','{}','{}'),",
                random_long(500),
                random_long(100),
                random_string(4),
                random_string(4)
            ));
    }

    // remove the last comma from the insert query and run
    insert_query.pop();
    let result = sqlx::query(&insert_query)
        .execute(&pool)
        .await;
    match result {
        Ok(_) => { }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}

pub fn generate_data(config: &config::Config, log: &Log) {
    if !config.data_generation.generate_data {
        log.info("generate_data flag is off, skipping data generation");
        return
    };

    log.debug("starting first data generation");
    generate_table(config, log, &config.database_1_config, true);

    log.debug("starting second data generation");
    generate_table(config, log, &config.database_2_config, true);

}

/// using thread_rng generate a random number between 1 and max
fn random_long(max: i32) -> i32 {
    thread_rng().gen_range(1..max)
}

/// using thread_rng and a vector of
/// characters generate a random string of length len
fn random_string(len: usize) -> String {
    let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::new();
    for _ in 0..len {
        result.push(
            characters[thread_rng().gen_range(0..characters.len())]
        );
    }
    result
}

// generates a new sqlite table from a passed in mysql row
pub async fn export_mysql_rows_to_sqlite_table(
    mysql_rows: &[MySqlRow],
    sqlite_pool: &Pool<sqlx::Sqlite>,
    table_name: &str,
    log: &Log
) -> bool {
    let mut create_query = format!("create table if not exists {} (", table_name);

    // for each column in the first mysql row generate the column name and type
    for column in mysql_rows[0].columns() {
        create_query.push_str(column.name());
        create_query.push(' ');
        create_query.push_str(&mysql_type_to_sqlite_type(column.type_info().name()));
        create_query.push(',');
    }

    // pop the last char off the string (,) and insert closing parens
    create_query.pop();
    create_query.push(')');

    log.debug(&format!("create query: {}", create_query));
    // execute and return the result
    let result = sqlx::query(create_query.as_str()).execute(sqlite_pool).await;
    match result {
        Ok(_) => true,
        Err(error) => {
            panic!("error occurred while generating the new sqlite table: {:?}", error);
        },
    }
}
