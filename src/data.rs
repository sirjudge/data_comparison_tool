use std::borrow::Borrow;

use async_std::task;
use sqlx::{ migrate::MigrateDatabase, mysql::MySqlPoolOptions, sqlite::SqlitePoolOptions, Error, MySql, Pool};

pub(crate) async fn create_sqlite_db(){
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
            println!("connected to sqlite db");
            let row = sqlx::query("SELECT 1")
                .fetch_one(&pool)
                .await
                .unwrap();
            
        }
        Err(error) => {
            println!("error: {}", error);
        }
    }
}


async fn connect_to_mysql_db() -> Result<Pool<MySql>, Error>{
    let db_url ="mysql://root:@0.0.0.0:3306";
    
    let result = MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(db_url).await;
    return result;
}

pub(crate) async fn pull_data(){
    let result = task::block_on(connect_to_mysql_db());
    match result {
        Ok(pool) => {
            let row = sqlx::query("SELECT 1")
                .fetch_one(&pool)
                .await
                .unwrap();
            println!("retrieved row: {:?}", row);
        }
        Err(error) => {
            println!("error: {}", error);
        }
    }
}
