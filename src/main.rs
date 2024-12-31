mod cli;
mod logger;

use std::env::current_dir;
use std::path::PathBuf;
use std::process::exit;

use git2::Repository;

use cli::Cli;
use ssc::should_skip_ci;
use logger::configure_logger;
use ssc::git::commits_range::CommitsRange;

fn main() {
    let cli: Cli = Cli::new();

    configure_logger(cli.verbosity());

    let working_directory: &PathBuf = &current_dir().unwrap();
    let repo = match Repository::open(&working_directory) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open repository {}", e),
    };

    let commits_range: CommitsRange = CommitsRange::resolve_commits_range(
        &repo,
        cli.remote(),
        cli.base_branch(),
        cli.base_ref(),
    );

    let status_code: i32 = should_skip_ci(
        &repo,
        cli.paths(),
        cli.cmd(),
        &commits_range,
    );

    exit(status_code);
}
