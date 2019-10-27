use std::process::Command;
use crate::git::asserter::assert_or_panic;

pub fn get_current_branch() -> String {
    let result = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Failed to determine current branch.")
    ;

    assert_or_panic(&result, &String::from("get branch"));

    let output: String = String::from_utf8(result.stdout).unwrap();

    return String::from(output.trim());
}

pub fn get_merge_base_commit(base_branch: &String) -> String {
    let result = Command::new("git")
        .arg("merge-base")
        .arg(base_branch)
        .arg("HEAD")
        .output()
        .expect("Failed to determine merge base.")
    ;

    assert_or_panic(&result, &String::from("git merge-base"));

    let output: String = String::from_utf8(result.stdout).unwrap();

    return String::from(output.trim());
}
