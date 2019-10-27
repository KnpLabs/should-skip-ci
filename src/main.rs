mod cli;

use cli::Cli;

fn main() {
    let mut cli: Cli = Cli::new();

    println!(
        "Dirs to inspect are {:?}.",
        cli.dirs()
    );
}
