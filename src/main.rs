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
    //TODO: Figure out absolute vs relative paths here
    // the path passed below is relative to wherever the command for CLI
    // is run from (ex. if in repo root, it'll properly identify the file
    // but if you `cd src` and then run it won't find the file)
    let toml_path = "src/comparison_default.toml";
    let config = Config::new(toml_path);
    let log = Log::new(&config);

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

    //TODO: Eventually need to come back and
    // maybe do something with this
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
