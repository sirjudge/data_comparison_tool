use data_comparison_tool::interface::{
    argument_parser,
    log,
    log_options::LogVerbosity
};

// todo: should have a method here that generates the data for the tables on start up and then does
// the same on tear down
pub fn setup() -> (argument_parser::Arguments, log::Log) {
    // init custom args for now
    let mut arguments = argument_parser::Arguments::new();
    arguments.tui = false;
    arguments.help = false;
    arguments.table_name_1 = "a_testTable1".to_string();
    arguments.table_name_2 = "a_testTable2".to_string();
    arguments.generate_data = true;
    arguments.clean = false;
    arguments.verbose = true;
    // init the log and run the processor
    let mut log = log::Log::new(&arguments);
    log.set_verbose(LogVerbosity::Debug);
    (arguments, log)
}

pub fn teardown() {
    panic!("teardown not implemented");
}
