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
    fn init_console_log_and_write(){
        let mut args = Arguments::new();
        args.log_output = LogOutput::Console;
        let log = Log::new(&args);
        log.info("info statement test");
        log.warn("info statement test");
        log.error("error statement tunwrap();est");
    }

    #[test]
    fn init_file_log_and_write(){
        let mut args = Arguments::new();
        args.log_output = LogOutput::File;
        let log = Log::new(&args);
        assert!(Path::new(&log.log_file_name).exists());
        log.info("info statement test");
        log.warn("info statement test");
        log.error("error statement tunwrap();est");
    }
}
