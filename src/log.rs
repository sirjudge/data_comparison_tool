use std::fs::File;
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
        println!("Log file already exists, please delete or rename the existing log file");
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
        let log_type = &args.log_output;

        Log {
            log_file_name,
            log_type: log_type.clone()
        }
    }

    pub fn info(&self, message: &str) {
        match self.log_type {
            LogOutput::StdOut |
            LogOutput::Console  => {
                println!("{}", message);
            }
            LogOutput::File => {
                let mut log_file = File::create(&self.log_file_name).unwrap();
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
                let mut log_file = File::create(&self.log_file_name).unwrap();
                let formatted_message = format!("ERROR: {}", message);
                match log_file.write_all(formatted_message.as_bytes()) {
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
