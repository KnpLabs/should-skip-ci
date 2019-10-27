use std::env::current_dir;
use std::path::PathBuf;
use structopt::StructOpt;

// RawCli represents the CLI args and options as passed on a shell.
#[derive(StructOpt)]
#[structopt(name = "ssc", about = "should-skip-ci")]
struct RawCli {
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

// The Cli struct represents the resolved CLI args and options.
pub struct Cli {
    raw_cli: RawCli,
    dirs: Vec<PathBuf>,
}

impl Cli {
    pub fn new() -> Self {
        let raw_cli: RawCli = RawCli::from_args();
        let mut dirs: Vec<PathBuf> = Vec::new();

        if raw_cli.dirs.is_empty() {
            dirs.push(current_dir().unwrap());
        } else {
            dirs = raw_cli.dirs.to_vec();
        }

        return Cli {
            raw_cli: raw_cli,
            dirs: dirs,
        }
    }

    pub fn dirs(&self) -> &Vec<PathBuf> {
        return &self.dirs;
    }

    pub fn pr_url(&self) -> &String {
        return &self.raw_cli.pr_url;
    }

    pub fn auth(&self) -> &String {
        return &self.raw_cli.auth;
    }

    pub fn from(&self) -> &String {
        return &self.raw_cli.from;
    }

    pub fn to(&self) -> &String {
        return &self.raw_cli.to;
    }

    pub fn cmd(&self) -> &String {
        return &self.raw_cli.cmd;
    }
}
