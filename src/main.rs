mod cli;
mod git;

use cli::Cli;
use git::commits_range::CommitsRange;
use git::commits_range::resolve_commits_range;
use git::path_inspector::has_changes_in_paths;

fn main() {
    let cli: Cli = Cli::new();

    let commits_range: CommitsRange = resolve_commits_range(&cli);

    if has_changes_in_paths(&commits_range, cli.paths()) {
        println!("CHANGES DETECTED");
    } else {
        println!("NO CHANGES DETECTED");
    }
}
