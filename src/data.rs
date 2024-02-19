use std::borrow::Borrow;
use sqlx::{ migrate::MigrateDatabase, mysql::{MySqlPoolOptions, MySqlRow}, sqlite::SqlitePoolOptions, Column, MySql, Pool, Row, TypeInfo};

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


pub(crate) async fn mysql_to_sqlite(mysql_rows: &Vec<MySqlRow>){

    // open a new sqlite connection and execute the create statment
    let sqlite_pool = get_sqlite_connection().await;
   
    // if we've built the new sqlite table 
    if create_new_sqlite_table(mysql_rows, &sqlite_pool).await {
        println!("created new sqlite table");
        
        // generate the insert query and run it
        let insert_query = create_sqlite_insert_query(mysql_rows);
        let result = sqlx::query(insert_query.as_str())
            .execute(&sqlite_pool)
            .await;
        match result {
            Ok(_) => {
                println!("inserted rows into sqlite table");
            }
            Err(error) => {
                panic!("error occurred while inserting rows into sqlite table: {:?}", error);
            }
        }
    }
    else {
        panic!("unable to create new sqlite table");
    } 

    drop(sqlite_pool);
}

// generates a new sqlite table from a passed in mysql row
async fn create_new_sqlite_table(mysql_rows: &Vec<MySqlRow>, sqlite_pool: &Pool<sqlx::Sqlite>) -> bool {
    // init insert query string
    let mut create_query  = "create table if not exists test_table (".to_string();
   
    // for each column in the first mysql row generate the column name and type
    for column in mysql_rows[0].columns(){
        create_query.push_str(&column.name());
        create_query.push_str(" ");
        let mysql_column_type = column.type_info().name();
        let sqlite_type = mysql_type_to_sqlite_type(mysql_column_type);
        create_query.push_str(&sqlite_type);
        create_query.push_str(",");
    }

    // pop the last char off the string (,) and insert closing parens
    create_query.pop();
    create_query.push_str(")");
    let result = sqlx::query(create_query.as_str())
        .execute(sqlite_pool)
        .await;
    match result {
        Ok(_) => {
            return true;
        }
        Err(error) => {
            panic!("error occurred while generating the new sqlite table: {:?}", error);
        }
    }
}

// generates a new sqlite insert query from a passed in mysql row
fn create_sqlite_insert_query(mysql_rows: &Vec<MySqlRow>) -> String {

    // for each row in mysql generate the insert statement
    let mut insert_query = "insert into test_table (".to_string();

    // generate the column insert list
    for column in mysql_rows[0].columns() {
        insert_query.push_str(&column.name());
        insert_query.push_str(",");
    }
    insert_query.pop();
    insert_query.push_str(") VALUES ");
    println!("insert query columns: {}", insert_query);
   

    // foreach row in the mysql result set generate the insert query
    for row in mysql_rows {
        let mut value_insert_string = "(".to_string();
        
        // generate the list of values to insert
        for column in row.columns() {
            let column_name = column.name();
            let column_type = column.type_info().name();
            match column_type {
                "INT" => {
                    let value: i32 = row.get(column_name);
                    value_insert_string.push_str(&value.to_string());
                    value_insert_string.push_str(",");
                }
                "VARCHAR" => {
                    let value: String = row.get(column_name);
                    value_insert_string.push_str("'");
                    value_insert_string.push_str(&value);
                    value_insert_string.push_str("',");
                }
                &_ => {
                    let value: String = row.get(column_name);
                    value_insert_string.push_str("'");
                    value_insert_string.push_str(&value);
                    value_insert_string.push_str("',");
                }
            }
        }
        // remove trailing comma and add closing parens
        value_insert_string.pop();
        value_insert_string.push_str("),");

        //println!("value insert string: {}", value_insert_string);
        insert_query.push_str(value_insert_string.as_str());
    }
    // remove trailing comma
    insert_query.pop();
    return insert_query;
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
