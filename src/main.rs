mod lib;

use std::env::current_dir;
use std::process::exit;

use cli::Cli;
use lib::should_skip_ci;
use logger::configure_logger;

fn main() {
    let cli: Cli = Cli::new();

    configure_logger(cli.verbosity());

    let status_code: i32 = should_skip_ci(
        &current_dir().unwrap(),
        cli.paths(),
        cli.cmd(),
        cli.remote(),
        cli.base_branch(),
    );

    exit(status_code);
}
