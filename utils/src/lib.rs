use std::process::Output;

pub fn assert_or_panic(result: &Output, name: &String) {
    if result.status.success() {
        return;
    }

    panic!(
        "error: {} command exited with status code {}. Reason: {}",
        name,
        result.status.code().unwrap(),
        String::from_utf8(result.stderr.clone()).unwrap(),
    );
}
