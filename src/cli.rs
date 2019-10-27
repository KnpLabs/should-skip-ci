use std::env::current_dir;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "ssc", about = "should-skip-ci")]
pub struct Cli {
    #[structopt(long = "dir", help = "The dir to inspect. Defaults to cwd. This arg can be specified multiple times to inspect multiple dirs.")]
    dirs: Vec<PathBuf>,
}

impl Cli {
    pub fn new() -> Self {
        return Cli::from_args();
    }

    pub fn dirs(&mut self) -> &Vec<PathBuf> {
        if self.dirs.is_empty() {
            self.dirs.push(current_dir().unwrap())
        }

        return &self.dirs
    }
}
