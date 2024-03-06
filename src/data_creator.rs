use rand::{ thread_rng, Rng};
use crate::data_querier::get_mysql_connection;

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
    
    for _i in 0..num_rows_to_generate {
        let insert_query = format!("INSERT INTO {}(randomNumber,secondRandomNumber,randomString,secondRandomString) 
            VALUES (?,?,?,?)", table_name);
        let result = sqlx::query(&insert_query)
            .bind(random_long(100))
            .bind(random_long(100))
            .bind(random_string(50))
            .bind(random_string(50))
            .execute(&pool)
            .await;
        match result {
            Ok(_) => { }
            Err(error) => {
                panic!("error: {:?}", error);
            }
        }
    }
}

fn random_long(max: i32) -> i32 {
    let n: i32 = thread_rng().gen_range(1..max);
    n
}

fn random_string(len: usize) -> String {
    let mut rng = thread_rng();
    let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::new();
    for _ in 0..len {
        let index = rng.gen_range(0..characters.len());
        result.push(characters[index]);
    }
    result
}

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

