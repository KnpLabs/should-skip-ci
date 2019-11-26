use std::env::current_dir;
use std::path::PathBuf;
use structopt::StructOpt;

// RawCli represents the CLI args and options as passed on a shell.
#[derive(StructOpt)]
#[structopt(name = "ssc", about = "should-skip-ci")]
struct RawCli {
    #[structopt(long = "path", help = "The path to inspect. Defaults to cwd. This arg can be specified multiple times to inspect multiple paths.")]
    paths: Vec<PathBuf>,

    #[structopt(long = "remote", default_value = "origin", help = "The name of the tracked repository.")]
    remote: String,

    #[structopt(long = "base-branch", default_value = "master", help = "The branch to use as a base to know from where the commit range starts (i.e. to find the merge base).")]
    base_branch: String,

    #[structopt(long = "cmd", help = "The command to use to skip the build.")]
    cmd: String,
}

// The Cli struct represents the resolved CLI args and options.
pub struct Cli {
    raw_cli: RawCli,
    paths: Vec<PathBuf>,
}

impl Cli {
    pub fn new() -> Self {
        let raw_cli: RawCli = RawCli::from_args();
        let mut paths: Vec<PathBuf> = Vec::new();

        if raw_cli.paths.is_empty() {
            paths.push(current_dir().unwrap());
        } else {
            paths = raw_cli.paths.to_vec();
        }

        return Cli {
            raw_cli: raw_cli,
            paths: paths,
        }
    }

    pub fn paths(&self) -> &Vec<PathBuf> {
        return &self.paths;
    }

    pub fn remote(&self) -> &String {
        return &self.raw_cli.remote;
    }

    pub fn base_branch(&self) -> &String {
        return &self.raw_cli.base_branch;
    }

    pub fn cmd(&self) -> &String {
        return &self.raw_cli.cmd;
    }
}
