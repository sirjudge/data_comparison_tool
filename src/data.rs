use sqlx::{migrate::MigrateDatabase, Sqlite};

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

pub(crate) async fn pull_data(){
}
