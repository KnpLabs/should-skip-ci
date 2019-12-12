use std::path::PathBuf;
use git::commits_range::CommitsRange;
use git::commits_range::resolve_commits_range;
use git::path_inspector::has_changes_in_paths;
use ci::run_stop_cmd;

pub fn should_skip_ci(
    paths: &Vec<PathBuf>,
    cmd: &String,
    remote: &String,
    base_branch: &String,
) -> i32 {
    let commits_range: CommitsRange = resolve_commits_range(
        remote,
        base_branch,
    );

    if has_changes_in_paths(&commits_range, paths) {
        println!("Changes detected. The CI build should continue.");

        return 0;
    } else {
        println!("No changes detected. Running the stop command.");

        return run_stop_cmd(cmd);
    }
}
