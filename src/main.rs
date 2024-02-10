use async_std::task::block_on;

mod data;


fn main() {
    parse_arguments();
    block_on(data::create_sqlite_db());

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(data::pull_data());
}

fn parse_arguments(){
    // split input args passed in
    let arguments = std::env::args();

    // loop over each argument
    for arg in arguments{
        println!("argument:{}",arg);
        // if arg contains '=' split the flag + value and print
        if arg.contains('='){
            let mut split_string = arg.split("=");
            let flag = split_string.next();
            let value = split_string.next();
            println!("Flag:{:?} Value:{:?}",flag,value);
        }
    }
}
