extern crate log;

use std::path::PathBuf;
use std::process::Command;
use log::{debug, info};

use utils::assert_or_panic;
use crate::commits_range::CommitsRange;

pub fn has_changes_in_paths(
    working_directory: &PathBuf,
    commits_range: &CommitsRange,
    paths: &Vec<PathBuf>
) -> bool {
    let mut cmd = Command::new("git");
    cmd
        .arg("log")
        .arg(commits_range.to_str())
        .arg("--stat")
        .args(paths)
        .current_dir(&working_directory)
    ;

    debug!("Running `{:?}`", cmd);

    let result = cmd
        .output()
        .expect("Failed to run git log command.")
    ;

    assert_or_panic(&result, &String::from("git log"));

    info!("{}", String::from_utf8(result.stdout.to_vec()).unwrap());

    return !result.stdout.is_empty();
}
