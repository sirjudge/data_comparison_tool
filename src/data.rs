use std::borrow::Borrow;
use sqlx::{ migrate::MigrateDatabase,mysql::{MySqlPoolOptions, MySqlRow}, sqlite::{SqlitePoolOptions, SqliteRow}, MySql, Pool};
use rand::{ Rng, thread_rng};

async fn get_sqlite_connection() -> Pool<sqlx::Sqlite>{
    let db_url = "sqlite://./db.sqlite3";

    if !sqlx::Sqlite::database_exists(db_url).await.unwrap(){
        println!("creating sqlite db");
        sqlx::Sqlite::create_database(db_url).await.unwrap(); 
    }

    let result = SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(db_url).await;
    match result {
        Ok(pool) => {
            return pool; 
        }
        Err(error) => {
            panic!("unable to connect to sqlite db {}", error);
        }
    }
}


pub(crate) async fn query_sqlite(query_string: &str) -> Vec<SqliteRow> {
    let pool = get_sqlite_connection().await;
    println!("connected to sqlite db");
    let rows = sqlx::query(query_string)
        .fetch_all(&pool)
        .await;
    match rows {
        Ok(rows) => {
            if rows.is_empty(){
                panic!("no rows returned");
            }
            else{
                return rows;
            }
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}


async fn get_mysql_connection(database_name: &str) -> Pool<MySql>{
    let db_url = format!("mysql://root:@0.0.0.0:3306/{}", database_name);
    let result = MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(db_url.borrow()).await;
    match result {
        Ok(pool) => {
            return pool;
        }
        Err(error) => {
            panic!("unable to connect to mysql db {}", error);
        }
    }
}

pub(crate) async fn query_mysql(query_string: &str) -> Vec<MySqlRow> {
    let pool = get_mysql_connection("test").await;
    let rows = sqlx::query(query_string)
        .fetch_all(&pool)
        .await;
    match rows{
        Ok(rows) => {
            if rows.is_empty(){
                panic!("no rows returned");
            }
            else {
                return rows;
            }
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}

pub(crate) async fn create_new_data(i: i32){
    let pool = get_mysql_connection("test").await;
    const CREATE_NEW_TABLE_QUERY: &str =
        "CREATE TABLE IF NOT EXISTS test_table 
        (
            id INT NOT NULL AUTO_INCREMENT,
            randomNumber INT NOT NULL,
            secondRandomNumber INT NOT NULL,
            randomString VARCHAR(255) NOT NULL,
            secondRandomString VARCHAR(255) NOT NULL, 
            PRIMARY KEY (id)
         )";
    let result = sqlx::query(CREATE_NEW_TABLE_QUERY)
        .execute(&pool)
        .await;
    match result {
        Ok(_) => {
            println!("created new table in mysql");
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
    
    for _i in 0..i {
        let insert_query = "INSERT INTO test_table (randomNumber,secondRandomNumber,randomString,secondRandomString) 
            VALUES (?,?,?,?)";
        let result = sqlx::query(insert_query)
            .bind(random_long(100))
            .bind(random_long(100))
            .bind(random_string(50))
            .bind(random_string(50))
            .execute(&pool)
            .await;
        match result {
            Ok(_) => {
                println!("inserted data into mysql");
            }
            Err(error) => {
                panic!("error: {:?}", error);
            }
        }
    }
}

fn random_long(max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(1..max);
    return n;
}

fn random_string(len: usize) -> String {
    let mut rng = thread_rng();
    let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::new();
    for _ in 0..len {
        let index = rng.gen_range(0..characters.len());
        result.push(characters[index]);
    }
    return result;
}
