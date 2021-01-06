# should-skip-ci

[![CircleCI](https://circleci.com/gh/KnpLabs/should-skip-ci/tree/master.svg?style=svg)](https://circleci.com/gh/KnpLabs/should-skip-ci/tree/master)

A CLI tool to skip a CI build that is not concerned by the latest changes.

*Table of contents:*

- [For who](#for-who)
- [CI setup](#ci-setup)
    - [Installation](#installation)
    - [Usage](#usage)
    - [Examples](#examples)
- [Limitations](#limitations)
- [Development](#development)

## For who

This tool is mostly used for mono git repos which are hosting multiple apps.

Let's say you have the following content in your git repository :

```
.
└── apps/
    ├── api/
    └── front/
```

Your `apps/api` dir contains your backend API and your `apps/front` dir
contains your frontend application.

You've written tests for both apps, but when you make changes on one of them,
you may not want to run the test suites for all apps (only tests covering your
changes are useful to run).

This tool can determine whether the CI job should be skipped or not, by looking
if the specified paths contains changes :

```bash
$ ssc --path apps/front --cmd=<stop_command>
```

this command will stop the CI build (by running the stop command) when there
are no changes on the specified path (`apps/front`).
When there are changes on the paths, the tool does nothing (and so the CI build
continues).

See the [expectations](/doc/arch/adr-001-expectations.md) to know why this tool
has been created, as long as
[this article](https://knplabs.com/en/blog/foss-project-explanation-of-should-skip-ci).

## CI setup

### Installation

Download the tool and make it executable :

```bash
$ sudo curl -sSL -o /usr/local/bin/ssc https://github.com/KnpLabs/should-skip-ci/releases/download/<version>/ssc-x86_64

$ sudo chmod +x /usr/local/bin/ssc
```

See the latest version in the [releases panel](https://github.com/KnpLabs/should-skip-ci/releases).

### Usage

```
USAGE:
    ssc [OPTIONS] --cmd <cmd>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v,-vv           Increases verbosity (-v for info, -vv for debug).

OPTIONS:
    --base-branch <base-branch>    The branch to use as a base to know from where the commit range starts (i.e. to
                                   find the merge base). [default: master]
    --cmd <cmd>                    The command to use to skip the build.
    --path <paths>...              The path to inspect. Defaults to cwd. This arg can be specified multiple times to
                                   inspect multiple paths.
    --remote <remote>              The name of the tracked repository. [default: origin]
```

### Examples

See our [CI config examples](/doc/examples/) to see how to integrate this tool
on your project.

## Limitations

This tool can be used in the following scenarii :

- on a pull request. You created a branch from your base branch (identified as
`origin/master` as default, but this identification is configurable via CLI
args, see the [usage](#usage) chapter). Your branch has `n` commits, so the tool
is using git to identify the range of commits in your PR (i.e. the first and
last commits). Then, the tool scans for changes on specified paths for this
range of commits.
- on a merge commit. When you run this tool on your base branch (defaults to
`origin/master`), the tool only scans the latest commit. This latest commit
should be a merge commit in order to identify all the changes that were brought
in this merge.

This tool can not be used for the following scenario :

- on a merge without a merge commit. As the tool scans for the latest commit
when you're on the base branch, this latest commit should be a merge commit.
If you merge a branch without creating a merge commit, the latest commit on the
base branch won't contain all the changes of your branch.

## Development

Open a shell into the `rust` container :

```bash
$ make shell
```

and then use `cargo` to launch the tool :

```bash
$ cargo run -- <ssc_options>
```

Also, see our [contributing guide](CONTRIBUTING.md).
