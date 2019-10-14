mod cli;

use cli::Cli;

fn main() {
    let cli: Cli = Cli::new();

    println!("Config file is {:?}.", cli.config_file().to_str().unwrap());
    println!("App name is {:?}.", cli.app_name());
}
