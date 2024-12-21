use std::io;
use data_comparison_tool::{
    processor,
    interface::{
        tui,
        argument_parser,
    },
    interface::log::Log
};

fn main() -> Result<(), io::Error> {
    // parse input arguments and initialize the log
    let args = argument_parser::Arguments::new();
    let log = Log::new(&args);

    // if help flag passed in don't do anything else
    if args.help {
        return Ok(());
    }

    // if the TUI flag is passed in run the terminal and early return
    if args.tui {
        let result = tui::run_terminal(&args, &log);
        ratatui::restore();
        return result;
    }

    //TODO: Eventually need to come back and
    // maybe do something with this
    let _comparison_data = processor::run_comparison(&args, &log);
    Ok(())
}

