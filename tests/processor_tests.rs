use data_comparison_tool::{
    processor,
};
pub mod test_utils;

/// a defaultly initialized argument should always pass
/// this test
#[test]
pub fn run_comparison_no_terminal_default_arg() {
    let (arguments, log) = test_utils::setup();
    let comparison_data = processor::run(&arguments, &log);
    // assert that the changed_rows is not empty
    assert!(!comparison_data.changed_rows.is_empty());
}
