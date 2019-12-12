mod lib;

use std::env::current_dir;
use std::process::exit;

use cli::Cli;
use lib::should_skip_ci;

fn main() {
    let cli: Cli = Cli::new();

    let status_code: i32 = should_skip_ci(
        &current_dir().unwrap(),
        cli.paths(),
        cli.cmd(),
        cli.remote(),
        cli.base_branch(),
    );

    exit(status_code);
}
