use std::path::PathBuf;
use std::process::Command;

use utils::assert_or_panic;

pub fn get_current_branch(
    working_directory: &PathBuf,
) -> String {
    let result = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(&working_directory)
        .output()
        .expect("Failed to determine current branch.")
    ;

    assert_or_panic(&result, &String::from("get branch"));

    let output: String = String::from_utf8(result.stdout).unwrap();

    return String::from(output.trim());
}

pub fn get_current_remote(
    working_directory: &PathBuf,
) -> String {
    let result = Command::new("git")
        .arg("remote")
        .arg("show")
        .current_dir(&working_directory)
        .output()
        .expect("Failed to determine current remote.")
    ;

    assert_or_panic(&result, &String::from("get remote"));

    let output: String = String::from_utf8(result.stdout).unwrap();

    return String::from(output.trim());
}

pub fn get_merge_base_commit(
    working_directory: &PathBuf,
    remote: &String,
    base_branch: &String,
) -> String {
    let result = Command::new("git")
        .arg("merge-base")
        .arg(format!(
            "{}/{}",
            remote,
            base_branch,
        ))
        .arg("HEAD")
        .current_dir(&working_directory)
        .output()
        .expect("Failed to determine merge base.")
    ;

    assert_or_panic(&result, &String::from("git merge-base"));

    let output: String = String::from_utf8(result.stdout).unwrap();

    return String::from(output.trim());
}
