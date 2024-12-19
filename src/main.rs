use std::io;

mod argument_parser;
mod data_comparer;
mod data_creator;
mod data_exporter;
mod data_querier;
mod log;
mod processor;
mod ui;

fn main() -> Result<(), io::Error> {
    // parse input arguments and initialize the log
    let args = argument_parser::Arguments::new();
    let log = log::Log::new(&args);

    // if help flag passed in don't do anything else
    if args.help {
        return Ok(());
    }

    // if the TUI flag is passed in run the terminal and early return
    if args.tui {
        let result = ui::run_terminal(&args, &log);
        ratatui::restore();
        return result;
    }

    let comparison_data = processor::run_comparison(&args, &log);
    Ok(())
}

#[cfg(test)]
pub mod main_tests {
    use super::*;

    /// a defaultly initialized argument should always pass
    /// this test
    #[test]
    pub fn run_comparison_no_terminal_default_arg() {
        let arguments = argument_parser::Arguments::new();
        let log = log::Log::new(&arguments);
        processor::run_comparison(&arguments, &log);
    }
}
