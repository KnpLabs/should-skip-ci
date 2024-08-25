use std::env::current_dir;
use std::path::PathBuf;
use clap::Parser;
use clap::builder::NonEmptyStringValueParser;
use clap::ArgAction::Count;

// RawCli represents the CLI args and options as passed on a shell.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RawCli {
    #[arg(
        long = "path",
        long_help = "The path to inspect. Defaults to cwd. This arg can be specified multiple times to inspect multiple paths. A path should point to any git node in the source tree.",
    )]
    paths: Vec<PathBuf>,

    #[arg(
        long,
        default_value = "origin",
        long_help = "The name of the tracked repository.",
    )]
    remote: String,

    #[arg(
        long = "base-branch",
        default_value = "master",
        long_help = "The branch name from where to look for changes. Not usable with `base-ref` arg.",
    )]
    base_branch: String,

    #[arg(
        long = "base-ref",
        visible_alias = "base-tag",
        conflicts_with = "base_branch",
        default_value = "",
        value_parser = NonEmptyStringValueParser::new(),
        long_help = "The ref (i.e. commit or tag) from where to look for changes. Not usable with `base-branch` arg.",
    )]
    base_ref: String,

    #[arg(
        long,
        long_help = "The command to use to skip the build.",
    )]
    cmd: String,

    // The number of occurrences of the `v/verbose` flag
    #[arg(
        short,
        long,
        action(Count),
        long_help = "Verbosity mode : -v (INFO), -vv (DEBUG)",
    )]
    verbosity: u8,
}

// The Cli struct represents the resolved CLI args and options.
pub struct Cli {
    raw_cli: RawCli,
    paths: Vec<PathBuf>,
}

impl Cli {
    pub fn new() -> Self {
        let raw_cli: RawCli = RawCli::parse();
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

    pub fn base_ref(&self) -> &String {
        return &self.raw_cli.base_ref;
    }

    pub fn cmd(&self) -> &String {
        return &self.raw_cli.cmd;
    }

    pub fn verbosity(&self) -> &u8 {
        return &self.raw_cli.verbosity;
    }
}
