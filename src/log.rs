use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::argument_parser::{Arguments, LogOutput};

/// creates a log file and returns the generated log file name
fn create_log_file() -> Result<String, std::io::Error> {
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
        let log_file_name = create_log_file().unwrap();
        //TODO: eventually should fix this
        let _log_type = &args.log_output;
        Log {
            log_file_name,
            log_type: LogOutput::File
        }
    }

    pub fn info(&self, message: &str) {
        match self.log_type {
            LogOutput::StdOut |
                LogOutput::Console  => {
                    println!("{}", message);
                }
            LogOutput::File => {
                let mut log_file: File;
                if Path::new(&self.log_file_name).exists() {
                    log_file = OpenOptions::new()
                        .append(true)
                        .open(&self.log_file_name)
                        .unwrap();
                }
                else {
                    log_file = File::create(&self.log_file_name).unwrap();
                }

                match log_file.write_all(message.as_bytes()){
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
        }
    }

    pub fn error(&self, message: &str) {
        match self.log_type {
            LogOutput::StdOut |
                LogOutput::Console  => {
                    eprintln!("{}", message);
                }
            LogOutput::File => {
                let mut log_file: File;
                if Path::new(&self.log_file_name).exists() {
                    log_file = OpenOptions::new()
                        .append(true)
                        .open(&self.log_file_name)
                        .unwrap();
                }
                else {
                    log_file = File::create(&self.log_file_name).unwrap();
                }

                match log_file.write_all(format!("ERROR: {}", message).as_bytes()) {
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
        }
    }
}


#[cfg(test)]
pub mod tests{
    use super::*;

    #[test]
    fn test_create_log_file(){
        let log_file_name_result = create_log_file();
        match log_file_name_result {
            Ok(log_file_name) => {
                assert!(Path::new(&log_file_name).exists());
            }
            Err(e) => {
                panic!("Error creating log file: {}", e);
            }
        }
    }


    #[test]
    fn init_file_log_and_(){
        let mut args = Arguments::new();
        args.log_output = LogOutput::File;
        let log = Log::new(&args);
        assert!(Path::new(&log.log_file_name).exists());
        log.info("info statement test");
        log.error("error statement tunwrap();est");
    }
}
