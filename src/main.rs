use std::io;
use data_comparison_tool::{
    interface::{
        config::Config,
        log::Log,
        tui
    },
    processor
};

fn main() -> Result<(), io::Error> {
    let config = Config::new();
    let log = Log::new(&config);

    // print configuration currently set
    if config.log_config.verbose {
        print_config(&config, &log);
    }

    // if help flag passed in don't do anything else
    if config.log_config.help{
        print_help();
        return Ok(());
    }

    // if the TUI flag is passed in run the terminal and early return
    if config.globals.tui {
        let result = tui::run_terminal(&config, &log);
        ratatui::restore();
        return result;
    }

    //TODO: Eventually need to come back and maybe do something with this
    let _comparison_data = processor::run(&config, &log);
    Ok(())
}

pub fn print_help(){
    println!("Help requested! This is a tool to help compare large data sets between mysql and sqlite");
    println!("Usage: data_comparison");
    println!("\t-h : print this help message");
    println!("\t-help : print this help message");
    println!("\t-tui : run with terminal ui");
    println!("\t-q1=<query> : specify a first mysql query to run");
    println!("\t-q2=<query> : specify a second mysql query to run");
    println!("\t-gen : generate new data in mysql");
    println!("\t-verbose : verbose output");
    println!("\t-version : print version information");
    println!("\t-c : clean sqlite database");
    println!("\t-t1=<table_name> : specify the name of the first table to compare");
    println!("\t-t2=<table_name> : specify the name of the second table to compare");
    println!("\t-in-memory : use an in memory sqlite database instead of file based");
    println!("\t-create-in-flight : create sqlite comparison files while in flight");
    println!("\t-auto-yes : automatically answer yes to all prompts");
    println!("\t-output=<output_file> : specify the name of the output csv file");
}

fn print_config(config: &Config, log: &Log) {
    log.info("Configuration:");
    log.info(&format!("database 1 db_name: {}", config.database_1_config.db_name));
    log.info(&format!("database 1 host: {}", config.database_1_config.db_host));
    log.info(&format!("database 2 db_name: {}", config.database_1_config.db_name));
    log.info(&format!("database 2 host: {}", config.database_1_config.db_host));
    log.info(&format!("table 1: {}", config.database_1_config.table_name));
    log.info(&format!("table 2: {}", config.database_2_config.table_name));
    log.info(&format!("output file name: {}", config.comparison_options.output_file_name));
    log.info(&format!("create sqlite comparison files: {}", config.comparison_options.create_sqlite_comparison_files));
    log.info(&format!("in memory sqlite: {}", config.comparison_options.in_memory_sqlite));
    log.info(&format!("clean: {}", config.comparison_options.clean));
    log.info(&format!("data generation: {}", config.data_generation.generate_data));
    log.info(&format!("number of rows to generate: {}", config.data_generation.number_of_rows_to_generate));
}
