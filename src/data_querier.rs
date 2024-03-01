use std::borrow::Borrow;
use sqlx::{ migrate::MigrateDatabase, mysql::{MySqlColumn, MySqlPoolOptions, MySqlRow}, sqlite::SqlitePoolOptions, Column, MySql, Pool, Row, TypeInfo};

pub struct TableData {
    pub table_name: String,
    pub columns: Vec<MySqlColumn>,
    pub primary_key: String
}

pub(crate) async fn get_mysql_table_data(table_name: &str) -> TableData {
    let pool = get_mysql_connection("test").await;
    let result = sqlx::query(&format!("select * from {} limit 1", table_name))
        .fetch_one(&pool)
        .await;
    match result {
        Ok(row) => {
            let columns = row.columns();
            let mut column_names = Vec::new();
            for column in columns {
                column_names.push(column.clone());
            }
            TableData {
                table_name: table_name.to_string(),
                columns: column_names,
                primary_key: "id".to_string()
            }
        }
        Err(error) => {
            panic!("error occurred while fetching table data: {:?}", error);
        }
    }
}

pub(crate) async fn get_sqlite_connection() -> Pool<sqlx::Sqlite>{
    let db_url = "sqlite://./db.sqlite3";

    // check if sqlite database exists and create it if it doesn't
    if !sqlx::Sqlite::database_exists(db_url).await.unwrap(){
        sqlx::Sqlite::create_database(db_url).await.unwrap(); 
        println!("database did not previously exist, created sqlite db");
    }

    // connect to the sqlite database and return the pool
    let result = SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(db_url).await;
    match result {
        Ok(pool) => {
            pool 
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
            pool
        }
        Err(error) => {
            panic!("unable to connect to mysql db {}", error);
        }
    }
}

pub(crate) async fn query_mysql(query_string: &str, database: &str) -> Vec<MySqlRow> {
    // open a connection to the test db and execute the query
    let pool = get_mysql_connection(database).await;
    let rows = sqlx::query(query_string)
        .fetch_all(&pool)
        .await;
    
    // if no errors return and rows isn't empty then return those rows, otherwise panic
    match rows{
        Ok(rows) => {
            if rows.is_empty(){
                panic!("no rows returned");
            }
            rows
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}

pub(crate) async fn mysql_table_to_sqlite_table(mysql_rows: &Vec<MySqlRow>, table_data: &TableData) {
    // open a new sqlite connection and execute the create statment
    let sqlite_pool = get_sqlite_connection().await;
   
    // if we've built the new sqlite table 
    if create_new_sqlite_table(mysql_rows, &sqlite_pool,&table_data.table_name).await {
        println!("created new sqlite table");
        
        // generate the insert query and run it
        let insert_query = create_sqlite_insert_query(mysql_rows, &table_data.table_name);
        let result = sqlx::query(insert_query.as_str())
            .execute(&sqlite_pool)
            .await;

        match result {
            Ok(_) => {}
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
async fn create_new_sqlite_table(mysql_rows: &Vec<MySqlRow>, sqlite_pool: &Pool<sqlx::Sqlite>, table_name: &str) -> bool {
    // extract table information
    // init insert query string
    let mut create_query  = format!("create table if not exists {} (", table_name );
   
    // for each column in the first mysql row generate the column name and type
    for column in mysql_rows[0].columns(){
        create_query.push_str(&column.name());
        create_query.push_str(" ");
        create_query.push_str(&mysql_type_to_sqlite_type(column.type_info().name()));
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
fn create_sqlite_insert_query(mysql_rows: &Vec<MySqlRow>, table_name: &str) -> String {

    // for each row in mysql generate the insert statement
    let mut insert_query = format!("insert into {} (" , table_name);

    // generate the column insert list
    for column in mysql_rows[0].columns() {
        insert_query.push_str(&column.name());
        insert_query.push_str(",");
    }

    insert_query.pop();
    insert_query.push_str(") VALUES ");
   
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
    insert_query
}

fn mysql_type_to_sqlite_type(mysql_type: &str) -> String 
{
    match mysql_type {
        "INT" => {
            "INTEGER".to_string()
        }
        "VARCHAR" => {
            "TEXT".to_string()
        }
        &_ => {
            "BLOB".to_string()
        }
    }
}
