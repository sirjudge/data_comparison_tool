use sqlx::{ migrate::MigrateDatabase, mysql::MySqlPoolOptions, sqlite::SqlitePoolOptions, Error, MySql, Pool};


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


pub(crate) async fn query_sqlite(query_string: &str){
    let pool = get_sqlite_connection().await;
    println!("connected to sqlite db");
    let row = sqlx::query(query_string)
        .fetch_one(&pool)
        .await
        .unwrap();
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

pub(crate) async fn query_mysql(query_string: &str){
    let pool = get_mysql_connection().await;
    let row = sqlx::query(query_string)
        .fetch_one(&pool)
        .await
        .unwrap();
}
