use std::io;
use data_comparison_tool::{
    interface::{
        log::Log,
        config::Config,
        tui
    },
    processor
};

fn main() -> Result<(), io::Error> {
    // parse input arguments and initialize the log
    // let config = argument_parser::Arguments::new();
    let config = Config::new_from_args();
    let log = Log::new(&config);

    // if help flag passed in don't do anything else
    if config.log_config.help{
        return Ok(());
    }

    // if the TUI flag is passed in run the terminal and early return
    if config.tui {
        let result = tui::run_terminal(&config, &log);
        ratatui::restore();
        return result;
    }

    //TODO: Eventually need to come back and
    // maybe do something with this
    let _comparison_data = processor::run(&config, &log);
    Ok(())
}

