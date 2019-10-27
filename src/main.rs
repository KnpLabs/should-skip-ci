mod cli;

use cli::Cli;

fn main() {
    let mut cli: Cli = Cli::new();

    println!(
        "Dirs to inspect are {:?}.",
        cli.dirs()
    );

    println!(
        "PR URL is {:?}",
        cli.pr_url()
    );

    println!(
        "auth is {:?}",
        cli.auth()
    );

    println!(
        "from is {:?}",
        cli.from()
    );

    println!(
        "to is {:?}",
        cli.to()
    );

    println!(
        "cmd is {:?}",
        cli.cmd()
    );
}
