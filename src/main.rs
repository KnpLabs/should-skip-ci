mod cli;
mod logger;

use std::env::current_dir;
use std::path::PathBuf;
use std::process::exit;

use cli::Cli;
use ssc::should_skip_ci;
use logger::configure_logger;
use ssc::git::commits_range::CommitsRange;

fn main() {
    let cli: Cli = Cli::new();

    configure_logger(cli.verbosity());

    let working_directory: &PathBuf = &current_dir().unwrap();

    let commits_range: CommitsRange = CommitsRange::resolve_commits_range(
        working_directory,
        cli.remote(),
        cli.base_branch(),
        cli.base_ref(),
    );

    let status_code: i32 = should_skip_ci(
        working_directory,
        cli.paths(),
        cli.cmd(),
        &commits_range,
    );

    exit(status_code);
}
