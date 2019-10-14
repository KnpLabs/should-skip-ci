use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "ssc", about = "should-skip-ci")]
struct Opt {
    #[structopt(short = "c", long = "config", help = "Config file (YAML).")]
    config_file: PathBuf,

    #[structopt(help = "The app name for which this CI job is for.")]
    app_name: String,
}

fn main() {
    let opt = Opt::from_args();

    println!("Config file is {:?}.", opt.config_file.to_str().unwrap());
    println!("App name is {:?}.", opt.app_name);
}
