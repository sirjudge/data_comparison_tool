use std::{borrow::BorrowMut, io};

mod processor;
mod argument_parser;
mod data_querier;
mod data_creator;
mod data_comparer;
mod data_exporter;
mod ui;
mod log;


fn main() -> Result<(), io::Error>{
    let args = argument_parser::Arguments::new();

    // if help is passed in we want to early return and not do anything else
    // helps prevent people from doing something after pushing the help flag
    if args.help {
        return Ok(());
    }

    // tui run the comaprison in the terminal
    // else just run the comparison here
    if args.tui {
        let result = ui::run_terminal(&args);
        ratatui::restore();
        return result;
    }
    else{
        processor::run_comparison(&args);
    }

    // finally return the result
    Ok(())
}

