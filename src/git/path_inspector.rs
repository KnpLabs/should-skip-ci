extern crate log;

use std::path::PathBuf;
use std::process::Command;
use log::{debug, info};

use crate::utils::assert_or_panic;
use super::commits_range::CommitsRange;

pub fn has_changes_in_paths(
    working_directory: &PathBuf,
    commits_range: &CommitsRange,
    paths: &Vec<PathBuf>
) -> bool {
    let mut cmd = Command::new("git");
    cmd
        .arg("diff")
        .arg("--stat")
        .arg(commits_range.from())
        .arg(commits_range.to())
        .args(paths)
        .current_dir(&working_directory)
    ;

    debug!("Running `{:?}`", cmd);

    let result = cmd
        .output()
        .expect("Failed to run git log command.")
    ;

    assert_or_panic(&result, &String::from("git diff"));

    info!(
        "Detected diff from {} to {} :\n{}",
        commits_range.from(),
        commits_range.to(),
        String::from_utf8(result.stdout.to_vec()).unwrap()
    );

    return !result.stdout.is_empty();
}
