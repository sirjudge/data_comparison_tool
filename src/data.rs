use std::borrow::Borrow;
use sqlx::{ migrate::MigrateDatabase, mysql::{MySqlPoolOptions, MySqlRow}, sqlite::{SqlitePoolOptions, SqliteRow}, Column, MySql, Pool, Row, TypeInfo};

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


pub(crate) async fn get_mysql_connection(database_name: &str) -> Pool<MySql>{
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


pub(crate) async fn mysql_to_sqlite(mysql_rows: Vec<MySqlRow>){
    // init insert query string
    let mut insert_query = "create table if not exists test_table (".to_string();
   
    // for each column in the first mysql row generate the column name and type
    for column in mysql_rows[0].columns() {
        insert_query.push_str(&column.name());
        insert_query.push_str(" ");
        let mysql_column_type = column.type_info().name();
        let sqlite_type = mysql_type_to_sqlite_type(mysql_column_type);
        println!("mysql_column_type:{} sqlite_type:{}",mysql_column_type,sqlite_type);
        insert_query.push_str(&sqlite_type);
        insert_query.push_str(",");
    }

    // pop the last char off the string (,) and insert closing parens
    insert_query.pop();
    insert_query.push_str(")");
    println!("insert_query:{}",insert_query);

    // open a new sqlite connection and execute the create statment
    let sqlite_pool = get_sqlite_connection().await;
    let result = sqlx::query(insert_query.as_str())
        .execute(&sqlite_pool)
        .await;
    match result {
        Ok(_) => {
            println!("created new table in sqlite");
        }
        Err(error) => {
            panic!("error occurred while generating the new sqlite table: {:?}", error);
        }
    }


    // for each row in mysql generate the insert statement
    // TODO: wrap this in a transaction later
    for row in mysql_rows {
        let mut insert_query = "insert into test_table (".to_string();
        let mut values = "values (".to_string();
        for column in row.columns() {
            insert_query.push_str(&column.name());
            insert_query.push_str(",");
            values.push_str("?,");
        }
        insert_query.pop();
        values.pop();
        insert_query.push_str(") ");
        values.push_str(")");
        insert_query.push_str(values.as_str());
        let mut bind_values = Vec::new();
        for column in row.columns() {
            let column_name = column.name();
            let column_type = column.type_info().name();
            match column_type {
                "INT" => {
                    let value: i32 = row.get(column_name);
                    bind_values.push(value.to_string());
                }
                "VARCHAR" => {
                    let value: String = row.get(column_name);
                    bind_values.push(value);
                }
                &_ => {
                    let value: String = row.get(column_name);
                    bind_values.push(value);
                }
            }
        }
    }
}


fn mysql_type_to_sqlite_type(mysql_type: &str) -> String 
{
    match mysql_type {
        "INT" => {
            return "INTEGER".to_string();
        }
        "VARCHAR" => {
            return "TEXT".to_string();
        }
        &_ => {
            return "TEXT".to_string();
        }
    }

}
