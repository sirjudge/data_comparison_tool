use sqlx::{ migrate::MigrateDatabase, mysql::{MySqlPoolOptions, MySqlRow}, sqlite::{SqlitePoolOptions, SqliteRow}, MySql, Pool, Row};


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


async fn get_mysql_connection() -> Pool<MySql>{
    let db_url ="mysql://root:@0.0.0.0:3306";
    let result = MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(db_url).await;

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
    let pool = get_mysql_connection().await;
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
