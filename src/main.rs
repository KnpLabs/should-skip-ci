mod cli;
mod git;

use cli::Cli;
use git::commits_range::CommitsRange;
use git::commits_range::resolve_commits_range;

fn main() {
    let cli: Cli = Cli::new();

    let commits_range: CommitsRange = resolve_commits_range(&cli);

    println!(
        "Range from is {:?}.",
        commits_range.from()
    );

    println!(
        "Range to is {:?}.",
        commits_range.to()
    );
}
