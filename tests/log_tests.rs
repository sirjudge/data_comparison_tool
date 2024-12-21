use data_comparison_tool::interface::{
    argument_parser::Arguments,
    log,
    log_options::LogOutput
};
use std::path::Path;

#[test]
fn init_console_log_and_write(){
    let mut args = Arguments::new();
    args.log_output = LogOutput::Console;
    let log = log::Log::new(&args);
    log.info("info statement test");
    log.warn("info statement test");
    log.error("error statement tunwrap();est");
}

#[test]
fn init_file_log_and_write(){
    let mut args = Arguments::new();
    args.log_output = LogOutput::File;
    let log = log::Log::new(&args);
    assert!(Path::new(&log.log_file_name).exists());
    log.info("info statement test");
    log.warn("info statement test");
    log.error("error statement tunwrap();est");
}
