use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "ssc", about = "should-skip-ci")]
pub struct Cli {
    #[structopt(short = "c", long = "config", help = "Config file (YAML).")]
    config_file: PathBuf,

    #[structopt(help = "The app name for which this CI job is for.")]
    app_name: String,
}

impl Cli {
    pub fn new() -> Self {
        return Cli::from_args();
    }

    pub fn config_file(&self) -> &PathBuf {
        return &self.config_file;
    }

    pub fn app_name(&self) -> &String {
        return &self.app_name;
    }
}
