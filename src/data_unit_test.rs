use rand::{ Rng, thread_rng};
use crate::data::get_mysql_connection;

pub(crate) async fn create_new_data(i: i32){
    let pool = get_mysql_connection("test").await;
    const CREATE_NEW_TABLE_QUERY: &str =
        "CREATE TABLE IF NOT EXISTS test_table 
        (
            id INT NOT NULL AUTO_INCREMENT,
            randomNumber INT NOT NULL,
            secondRandomNumber INT NOT NULL,
            randomString VARCHAR(255) NOT NULL,
            secondRandomString VARCHAR(255) NOT NULL, 
            PRIMARY KEY (id)
         )";
    let result = sqlx::query(CREATE_NEW_TABLE_QUERY)
        .execute(&pool)
        .await;
    match result {
        Ok(_) => {
            println!("created new table in mysql");
        }
        Err(error) => {
            panic!("error: {:?}", error);
        }
    }
    
    for _i in 0..i {
        let insert_query = "INSERT INTO test_table (randomNumber,secondRandomNumber,randomString,secondRandomString) 
            VALUES (?,?,?,?)";
        let result = sqlx::query(insert_query)
            .bind(random_long(100))
            .bind(random_long(100))
            .bind(random_string(50))
            .bind(random_string(50))
            .execute(&pool)
            .await;
        match result {
            Ok(_) => {
                println!("inserted data into mysql");
            }
            Err(error) => {
                panic!("error: {:?}", error);
            }
        }
    }
}

fn random_long(max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(1..max);
    return n;
}

fn random_string(len: usize) -> String {
    let mut rng = thread_rng();
    let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::new();
    for _ in 0..len {
        let index = rng.gen_range(0..characters.len());
        result.push(characters[index]);
    }
    return result;
}

