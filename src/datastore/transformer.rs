use crate::{
    interface::log::Log,
    models::table_data::TableData,
    datastore::{
        sqlite,
        generator
    }
};

use sqlx::{
    mysql::MySqlRow,
    Row,
    Column,
    TypeInfo
};

/// Converts a batch of MySql rows to a sqlite new sqlite table
/// and inserts the rows into the new table
pub(crate) async fn mysql_table_to_sqlite_table(
    mysql_rows: &Vec<MySqlRow>,
    table_data: &TableData,
    log: &Log,
) {
    // open a new sqlite connection and execute the create statment
    let sqlite_pool = sqlite::get_connection(log).await;

    // if we've built the new sqlite table
    if generator::export_mysql_rows_to_sqlite_table(mysql_rows, &sqlite_pool, &table_data.table_name, log).await {
        log.info(&format!("created new sqlite table: {}", &table_data.table_name));

        // generate the insert query and run it
        let insert_query = create_sqlite_insert_query_from_mysql_row(mysql_rows, &table_data.table_name);
        let result = sqlx::query(insert_query.as_str()).execute(&sqlite_pool).await;

        match result {
            Ok(_) => {},
            Err(error) => {
                panic!("error occurred while inserting rows into sqlite table: {:?}", error);
            },
        }
    } else {
        panic!("unable to create new sqlite table");
    }
}

// generates a new sqlite insert query from a passed in mysql row
fn create_sqlite_insert_query_from_mysql_row(mysql_rows: &Vec<MySqlRow>, table_name: &str) -> String {
    // for each row in mysql generate the insert statement
    let mut insert_query = format!("insert into {} (", table_name);

    // generate the column insert list
    for column in mysql_rows[0].columns() {
        insert_query.push_str(column.name());
        insert_query.push(',');
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

            // TODO: add support for other column types here
            match column_type {
                "INT" => {
                    let value: i32 = row.get(column_name);
                    value_insert_string.push_str(&value.to_string());
                    value_insert_string.push(',');
                },
                "VARCHAR" => {
                    let value: String = row.get(column_name);
                    value_insert_string.push('\'');
                    value_insert_string.push_str(&value);
                    value_insert_string.push_str("',");
                },
                &_ => {
                    let value: String = row.get(column_name);
                    value_insert_string.push('\'');
                    value_insert_string.push_str(&value);
                    value_insert_string.push_str("',");
                },
            }
        }

        // remove trailing comma and add closing parens
        value_insert_string.pop();
        value_insert_string.push_str("),");

        insert_query.push_str(value_insert_string.as_str());
    }
    // remove trailing comma
    insert_query.pop();
    insert_query
}

pub fn mysql_type_to_sqlite_type(mysql_type: &str) -> String {
    //TODO: add support for other column types here
    match mysql_type {
        "INT" => "INTEGER".to_string(),
        "VARCHAR" => "TEXT".to_string(),
        &_ => "BLOB".to_string(),
    }
}
