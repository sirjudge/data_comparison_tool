#[cfg(test)]
pub mod processor_test {
    use super::*;

    /// a defaultly initialized argument should always pass
    /// this test
    #[test]
    pub fn run_comparison_no_terminal_default_arg() {
        let arguments = argument_parser::Arguments::new();
        let log = log::Log::new(&arguments);
        processor::run_comparison(&arguments, &log);
    }
}
