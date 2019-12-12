use std::path::PathBuf;
use ssc::should_skip_ci;

#[test]
fn it_tests() {
    assert_eq!(0, should_skip_ci(
        &vec![PathBuf::from(".")],
        &String::from("true"),
        &String::from("origin"),
        &String::from("master"),
    ))
}
