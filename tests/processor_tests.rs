use data_comparison_tool::{
    interface::{
        argument_parser,
        log,
        log_options::LogVerbosity
    },
    processor
};

/// a defaultly initialized argument should always pass
/// this test
#[test]
pub fn run_comparison_no_terminal_default_arg() {
    // init custom args for now
    let mut arguments = argument_parser::Arguments::new();
    arguments.tui = false;
    arguments.help = false;
    arguments.table_name_1 = "0_testTable1".to_string();
    arguments.table_name_2 = "0_testTable2".to_string();
    arguments.generate_data = false;
    arguments.clean = false;
    arguments.verbose = true;

    // init the log and run the processor
    let mut log = log::Log::new(&arguments);
    log.set_verbose(LogVerbosity::Debug);
    processor::run(&arguments, &log);
}

