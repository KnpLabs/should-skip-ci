use std::path::PathBuf;
use std::process::Command;

use utils::assert_or_panic;
use crate::commits_range::CommitsRange;

pub fn has_changes_in_paths(
    commits_range: &CommitsRange,
    paths: &Vec<PathBuf>
) -> bool {
    let result = Command::new("git")
        .arg("log")
        .arg(commits_range.to_str())
        .args(paths)
        .output()
        .expect("Failed to run git log command.")
    ;

    assert_or_panic(&result, &String::from("git log"));

    return !result.stdout.is_empty();
}
