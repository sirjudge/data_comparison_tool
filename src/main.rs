fn main() {
    create_sqlite_db();
}

fn create_sqlite_db(){
    let connection = sqlite::open("comparison.db");
    println!("opened new connection");
    let query = "select 1";
    connection.unwrap().execute(query).expect("oopsie");
    println!("executed query")
}