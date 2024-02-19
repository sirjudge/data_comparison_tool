pub struct Arguments {
    pub mysql_query: String, 
    pub generate_data: bool,
    pub verbose: bool,
    pub version: bool,
    pub help: bool
}

pub(crate) fn parse_arguments() -> Arguments{
    // split input args passed in
    let std_in_args = std::env::args();

    // init argument struct
    let mut return_arguments = Arguments{
        mysql_query: "select 1 as number".to_string(),
        generate_data: false,
        verbose: false,
        version: false,
        help: false
    };
   
    // loop over each argument
    // first argument is the name of the program so we can ignore it
    for arg in std_in_args{
        println!("argument:{}",arg);
        // if arg contains '=' split the flag + value and print
        if arg.contains('='){
            let mut split_string = arg.split("=");
            let flag = split_string.next();
            let value = split_string.next();
            std::env::set_var(flag.unwrap(), value.unwrap());
            match flag.unwrap() {
                "-q" => {
                    return_arguments.mysql_query = value.unwrap().to_string();
                }
                &_ => {
                    println!("Unknown argument:{}",arg);
                }
            }
        }
        else {
            match arg.as_str() {
                "-h" => {
                    return_arguments.help = true;
                }
                "-v" => {
                    return_arguments.verbose = true;
                }
                "-g" => {
                    return_arguments.generate_data = true;
                }
                &_ => {
                    println!("Unknown argument:{}",arg);
                }
            }
        }
    }
    return return_arguments;
}
