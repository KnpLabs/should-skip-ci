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
Usage: ssc [OPTIONS] --cmd <CMD>

Options:
      --path <PATHS>
          The path to inspect. Defaults to cwd.
          This arg can be specified multiple times to inspect multiple paths.
          A path should point to any git node in the source tree.

      --remote <REMOTE>
          The name of the tracked repository.

          [default: origin]

      --base-branch <BASE_BRANCH>
          The branch name from where to look for changes.
          Not usable with `base-ref` arg.

          [default: master]

      --base-ref <BASE_REF>
          The ref (i.e. commit or tag) from where to look for changes.
          Not usable with `base-branch` arg.

          [default: ]
          [aliases: base-tag]

      --cmd <CMD>
          The command to use to skip the build.

  -v, --verbosity...
          Verbosity mode : -v (INFO), -vv (DEBUG)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Examples

See our [CI config examples](/doc/examples/) to see how to integrate this tool
on your project.

## Limitations

### Platform

Currently, only the GNU/Linux x86_64 platform is supported. However, suggestions
for other platforms are welcome.

### VCS (Version Control System)

Currently, only [git](https://git-scm.com/) VCS is supported. However,
suggestions for other VCSs are welcome.

Using the [git bindings](https://crates.io/crates/git2) API does not seem to
[improve the tool](https://github.com/KnpLabs/should-skip-ci/pull/44), so we can
rely on git commands execution.

### Use cases

This tool can be used in the following scenarios :

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
- to compare with a tag. In order to see if you have changes in the provided
paths since this tag (can be a tag or a commit hash, see the `base-ref` CLI
arg).

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
## Contribution guide
Also, see our [contributing guide](CONTRIBUTING.md).

## Maintainers
[nm2107](https://github.com/nm2107)
[jaljo](https://github.com/jaljo)
