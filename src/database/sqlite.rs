use crate::log::Log;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::SqlitePoolOptions,
    Pool,
};

/// open a connection to the sqlite database
pub(crate) async fn get_sqlite_connection(log: &Log) -> Pool<sqlx::Sqlite> {
    let db_url = "sqlite://./db.sqlite3";
    // check if sqlite database exists and create it if it doesn't
    if !sqlx::Sqlite::database_exists(db_url).await.unwrap() {
        sqlx::Sqlite::create_database(db_url).await.unwrap();
        log.info(&format!("database did not previously exist, created sqlite db"));
    }

    // connect to the sqlite database and return the pool
    let result = SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(db_url)
        .await;
    match result {
        Ok(pool) => pool,
        Err(error) => {
            panic!("unable to connect to sqlite db {}", error);
        },
    }
}

