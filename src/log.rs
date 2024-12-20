use std::{
    fs::{File, OpenOptions},
    io::{Write,Error},
    path::Path
};
use crate:: models::argument_parser::{Arguments, LogOutput};

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
    log_type: LogOutput
}

impl Log {
    pub fn new(args:&Arguments) -> Log {
        Log {
            log_file_name: create_log_file().unwrap(),
            log_type: args.log_output.clone()
        }
    }

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

    pub fn info(&self, message: &str) {
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

    pub fn warn(&self, message: &str) {
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


