mod cli;
mod git;
mod ci;

use cli::Cli;
use git::commits_range::CommitsRange;
use git::commits_range::resolve_commits_range;
use git::path_inspector::has_changes_in_paths;
use ci::run_stop_cmd;
use std::process::exit;

fn main() {
    let cli: Cli = Cli::new();

    let commits_range: CommitsRange = resolve_commits_range(&cli);

    if has_changes_in_paths(&commits_range, cli.paths()) {
        println!("Changes detected. The CI build should continue.");
    } else {
        println!("No changes detected. Running the stop command.");

        let code = run_stop_cmd(cli.cmd());

        exit(code);
    }
}
