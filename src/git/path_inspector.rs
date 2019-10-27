use std::path::PathBuf;
use std::process::Command;
use crate::git::commits_range::CommitsRange;

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

    return !result.stdout.is_empty();
}
