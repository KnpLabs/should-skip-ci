use std::env::current_dir;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "ssc", about = "should-skip-ci")]
pub struct Cli {
    #[structopt(long = "dir", help = "The dir to inspect. Defaults to cwd. This arg can be specified multiple times to inspect multiple dirs.")]
    dirs: Vec<PathBuf>,

    #[structopt(long = "pr-url", default_value = "", help = "The app name for which this CI job is for.")]
    pr_url: String,
}

impl Cli {
    pub fn new() -> Self {
        return Cli::from_args();
    }

    pub fn dirs(&mut self) -> &Vec<PathBuf> {
        if self.dirs.is_empty() {
            self.dirs.push(current_dir().unwrap())
        }

        return &self.dirs;
    }

    pub fn pr_url(&self) -> &String {
        return &self.pr_url;
    }
}
