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
        config.print_config(&log)
    }

    // if help flag passed in don't do anything else
    if config.log_config.help {
        config.print_help(&log);
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
