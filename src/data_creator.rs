use rand::{ thread_rng, Rng};
use crate::data_querier::get_mysql_connection;

/// Create a new table in the mysql database and populate it with random data
pub(crate) async fn create_new_data(num_rows_to_generate: i32, table_name: &str){
    //TODO: eventually should really introduce some kind of JSON schema input
    let pool = get_mysql_connection("test").await;
    let create_new_table_query = format!(
        "CREATE TABLE IF NOT EXISTS {}
        (
            id INT NOT NULL AUTO_INCREMENT,
            randomNumber INT NOT NULL,
            secondRandomNumber INT NOT NULL,
            randomString VARCHAR(255) NOT NULL,
            secondRandomString VARCHAR(255) NOT NULL,
            PRIMARY KEY (id)
        )", table_name);
    let result = sqlx::query(&create_new_table_query)
        .execute(&pool)
        .await;
    match result {
        Ok(_) => {
            println!("created new mysql table: {}", table_name);
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }

    let mut insert_query =
        format!(
            "INSERT INTO {}
            (randomNumber,secondRandomNumber,randomString,secondRandomString)
            VALUES ", table_name);
    for _i in 0..num_rows_to_generate {
        insert_query.push_str(
            &format!(
                "({},'{}','{}','{}'),",
                random_long(500),
                random_long(500),
                random_string(25),
                random_string(25)
                ));
    }

    // remove the last comma from the insert query and run
    insert_query.pop();
    let result = sqlx::query(&insert_query)
        .execute(&pool)
        .await;
    match result {
        Ok(_) => { }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
}


/// using thread_rng generate a random number between 1 and max
fn random_long(max: i32) -> i32 {
    thread_rng().gen_range(1..max)
}

/// using thread_rng and a vector of characters generate a random string of length len
fn random_string(len: usize) -> String {
    let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::new();
    for _ in 0..len {
        result.push(characters[thread_rng().gen_range(0..characters.len())]);
    }
    result
}

/// Cleans up all sqlite files inside the current executing directory
pub(crate) async fn clear_sqlite_data(){
    // get all files in the current directory
    let files = std::fs::read_dir(".").unwrap();
    for file in files{
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name.ends_with(".sqlite"){
            std::fs::remove_file(file_name).unwrap();
        }
    }
}

