use rand::{ thread_rng, Rng};
use crate::{
    datastore::{
        mysql::get_mysql_connection,
        transformer::mysql_type_to_sqlite_type
    },
    interface::log::Log,
};
use sqlx::{
    Pool,
    mysql::MySqlRow,
    Row,
    Column,
    TypeInfo
};
/// Create a new table in the mysql database and populate it with random data
pub(crate) async fn create_new_mysql_data(num_rows_to_generate: i32, table_name: &str, log: &Log){
    let pool = get_mysql_connection("test", log).await;
    let create_new_table_query = format!(
        "CREATE TABLE IF NOT EXISTS {}
        (
            id INT NOT NULL AUTO_INCREMENT,
            randomNumber INT NOT NULL,
            secondRandomNumber INT NOT NULL,
            randomString VARCHAR(255) NOT NULL,
            secondRandomString VARCHAR(255) NOT NULL,
            PRIMARY KEY (id)
        )", table_name);

    let result = sqlx::query(&create_new_table_query)
        .execute(&pool)
        .await;
    match result {
        Ok(_) => {
            log.info(&format!("created new mysql table: {}", table_name));
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }

    let mut insert_query =
        format!(
            "INSERT INTO {}
            (randomNumber,secondRandomNumber,randomString,secondRandomString)
            VALUES ", table_name
        );

    for _i in 0..num_rows_to_generate {
        insert_query.push_str(
            &format!(
                "({},'{}','{}','{}'),",
                random_long(500),
                random_long(500),
                random_string(25),
                random_string(25)
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


/// using thread_rng generate a random number between 1 and max
fn random_long(max: i32) -> i32 {
    thread_rng().gen_range(1..max)
}

/// using thread_rng and a vector of characters generate a random string of length len
fn random_string(len: usize) -> String {
    let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::new();
    for _ in 0..len {
        result.push(characters[thread_rng().gen_range(0..characters.len())]);
    }
    result
}

// generates a new sqlite table from a passed in mysql row
pub async fn create_new_sqlite_table(
    mysql_rows: &[MySqlRow],
    sqlite_pool: &Pool<sqlx::Sqlite>,
    table_name: &str,
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

    // execute and return the result
    let result = sqlx::query(create_query.as_str()).execute(sqlite_pool).await;
    match result {
        Ok(_) => true,
        Err(error) => {
            panic!("error occurred while generating the new sqlite table: {:?}", error);
        },
    }
}
