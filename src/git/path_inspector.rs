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

    if !result.status.success() {
        panic!(format!(
            "error: git log command exited with status code {}. Reason: {}",
            result.status.code().unwrap(),
            String::from_utf8(result.stderr).unwrap(),
        ));
    }

    return !result.stdout.is_empty();
}
