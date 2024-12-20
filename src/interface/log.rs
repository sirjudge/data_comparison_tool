use std::{
    fs::{File, OpenOptions},
    io::{Write,Error},
    path::Path
};
use crate::{
    models::argument_parser::{Arguments, LogOutput},
    interface::log_options::LogVerbosity
};

/// creates a log file and returns the generated log file name
fn create_log_file() -> Result<String, Error> {
    let datetime_string =
        chrono::Local::now().to_string()
        .replace(" ", "_")
        .replace(":", "_");
    let log_file_name = format!("data_comparison_{}.log", datetime_string);

    if Path::new(&log_file_name).exists(){
        println!("log file already exists: {}", log_file_name);
        return Ok(log_file_name);
    }

    let mut log_file = File::create(&log_file_name)?;
    log_file.flush()?;
    Ok(log_file_name)
}


pub struct Log {
    log_file_name: String,
    log_type: LogOutput,
    verbose: LogVerbosity
}

impl Log {
    pub fn new(args:&Arguments) -> Log {
        Log {
            log_file_name: create_log_file().unwrap(),
            log_type: args.log_output.clone(),
            verbose: LogVerbosity::Info
        }
    }

    /// opens a new file in append mode if the file exists or create a new
    /// file if it doesn't
    fn open_file(&self) -> File {
        if Path::new(&self.log_file_name).exists() {
                OpenOptions::new()
                .append(true)
                .open(&self.log_file_name)
                .unwrap()
        }
        else {
                File::create(&self.log_file_name).unwrap()
        }
    }

    /// appends a given string to the file found in the path provided
    fn append_to_file(&self,log_message: &str, log_file: &mut File) {
        match log_file.write_all(log_message.as_bytes()) {
            Ok(_) => {
                match log_file.flush() {
                    Ok(_) => {}
                    Err(e) => {
                        panic!("Error flushing log file: {}", e);
                    }
                }
            }
            Err(e) => {
                panic!("Error writing to log file: {}", e);
            }
        }

    }

    /// print debug message to the configured output if the verbosity is set
    /// to debug or above
    pub fn debug(&self, message: &str) {
        if self.verbose.clone() as i32 > LogVerbosity::Debug as i32 {
            return;
        }

        match self.log_type {
            LogOutput::StdOut | LogOutput::Console  => {
                println!("<DEBUG>{}", message);
            }
            LogOutput::File => {
                let mut log_file = self.open_file();
                self.append_to_file(&format!("<DEBUG>{}\n",message), &mut log_file);
            }
        }
    }

    /// print info message to the configured output if the verbosity is set
    /// to info or above
    pub fn info(&self, message: &str) {
        if self.verbose.clone() as i32 > LogVerbosity::Info as i32 {
            return;
        }

        match self.log_type {
            LogOutput::StdOut | LogOutput::Console  => {
                println!("<INFO>{}", message);
            }
            LogOutput::File => {
                let mut log_file = self.open_file();
                self.append_to_file(&format!("<INFO>{}\n",message), &mut log_file);
            }
        }
    }

    /// print warning message to the configured output if the verbosity is set
    /// to warning or above
    pub fn warn(&self, message: &str) {
        if self.verbose.clone() as i32 > LogVerbosity::Warning as i32 {
            return;
        }
        match self.log_type {
            LogOutput::StdOut |
                LogOutput::Console  => {
                    println!("<WARNING>{}", message);
                }
            LogOutput::File => {
                let mut log_file = self.open_file();
                self.append_to_file(&format!("<WARNING>{}\n",message), &mut log_file);
            }
        }
    }

    /// print error message to the configured output
    /// note: we always want to print errors so don't need to check verbosity at this point
    pub fn error(&self, message: &str) {

        match self.log_type {
            LogOutput::StdOut |
                LogOutput::Console  => {
                    eprintln!("<ERROR>{}", message);
                }
            LogOutput::File => {
                let mut log_file = self.open_file();
                self.append_to_file(&format!("<ERROR>{}\n",message), &mut log_file);
            }
        }
    }
}


