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
    let args = argument_parser::Arguments::new();

    // if help is passed in we want to early return and not do anything else
    // helps prevent people from doing something after pushing the help flag
    if args.help {
        return Ok(());
    }

    // tui run the comparison in the terminal
    // else just run the comparison here
    if args.tui {
        let result = ui::run_terminal(&args);
        ratatui::restore();
        return result;
    }
    else {
        processor::run_comparison(&args);
    }

    // finally return the result
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
        processor::run_comparison(&arguments);
    }
}
