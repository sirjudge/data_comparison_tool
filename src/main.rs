use async_std::task::block_on;
use sqlx::Row;

mod data;

fn main() {
    parse_arguments();

    let query = "select 1 as number";
    let sqlite_row = block_on(data::query_sqlite(query));
    println!(" returned # of sqlite rows: {:?}",sqlite_row.len());

    let query = "select 1 as number";
    let mysql_rows = block_on(data::query_mysql(query));
    println!(" returned # of mysql rows: {:?}",mysql_rows.len());
}

fn parse_arguments(){
    // split input args passed in
    let arguments = std::env::args();

    // loop over each argument
    // first argument is the name of the program so we can ignore it
    for arg in arguments{
        println!("argument:{}",arg);
        // if arg contains '=' split the flag + value and print
        if arg.contains('='){
            let mut split_string = arg.split("=");
            let flag = split_string.next();
            let value = split_string.next();
            std::env::set_var(flag.unwrap(), value.unwrap());
            println!("Flag:{:?} Value:{:?}",flag,value);
        }
        else {
            match arg.as_str() {
                "-h" => {
                    println!("Tool to do cool datat things");
                }
                "-v" => {
                    println!("Version: 0.1.0");
                }
                &_ => {
                    println!("Unknown argument:{}",arg);
                }
            }
        }
    }
}
