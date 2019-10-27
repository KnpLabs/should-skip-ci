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

    #[structopt(long = "auth", default_value = "", help = "The credentials to use to connect to the PR URL.")]
    auth: String,

    #[structopt(long = "from", default_value = "HEAD~1", help = "From commit hash. Defaults to HEAD~1 overriden by the first PR commit when the --pr-url flag is provided.")]
    from: String,

    #[structopt(long = "to", default_value = "HEAD", help = "The end commit hash. Defaults to HEAD.")]
    to: String,

    #[structopt(long = "cmd", help = "The command to use to skip the build.")]
    cmd: String,
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

    pub fn auth(&self) -> &String {
        return &self.auth;
    }

    pub fn from(&self) -> &String {
        return &self.from;
    }

    pub fn to(&self) -> &String {
        return &self.to;
    }

    pub fn cmd(&self) -> &String {
        return &self.cmd;
    }
}
