use async_std::task;
use sqlx::{migrate::MigrateDatabase, mysql::MySqlPoolOptions, Error, MySql, MySqlPool, Pool, Sqlite};


pub(crate) async fn create_sqlite_db(){
    let db_url = "sqlite:db.sqlite3";
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}


async fn connect_to_mysql_db() -> Result<Pool<MySql>, Error>{
    let db_url ="mysql://root:verysecurepassword@0.0.0.0:3306";  
    return MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .max_connections(5)
        .connect(db_url)
        .await;
}

pub(crate) async fn pull_data(){
    println!("connecting to mysql db");
    let result = task::block_on(connect_to_mysql_db());
    match result {
        Ok(pool) => {
            println!("connected to mysql db, attempting to select");
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
