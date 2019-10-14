# ADR 001 : Expectations

## Context

We sometimes use a single git repository to host multiple apps, for instance :

```
├── apps/
│   ├── api/      # the backend REST API
│   ├── cli/      # a CLI tool for the project
│   └── frontend/ # the frontend app, to be used in a web browser
└── README.md
```

When a CI pipeline is set on these repositories, any change is triggering a new
CI build. During the build, all the apps are tested.

However, the changes can be related to a single app only, so it would not make
sense to run the test suite for the other apps that are not impacted by the
changes.

On the other hand, even if the changes are on a single app, the developer may
want to run the test suite of other apps too that are depending on the first
one, to ensure the integration of the changes.

Finding a way to skip the CI builds when the changes aren't related or aren't
impacting an app would save build resources and build time.

## Decision

We could write a tool that would use the versionning control system (e.g. git)
to determine what are the apps impacted by the changes.

Then, on the CI job, we could skip the current app job when the changes aren't
related or aren't impacting this app.

### Workflow

The tool should be used to indicate if the build for a given app should be
skipped :

```bash
# ssc : should skip ci
$ ssc <app_name>
```

When the job of the given app should be skipped, the tool will call the command
to skip the job (the command depends on the CI platform). When the job should
not be skipped, the tool will simply exit `0` without calling any commands.

On pull requests (aka merge requests), we can detect the changes contained in
the PR by looking at the commits of the PR.

For commits on the `master` branch, we could only look to the latest commit
to detect the changes, as the latest commit should be a merge commit (i.e. when
the developer merges a PR).

In order to determine the apps dependency graph, we could use a simple yaml
file :

```yaml
apps:

  api:
    path: apps/api

  cli:
    path: apps/cli

  frontend:
    path: apps/frontend
    depends_on:
      - api
```

When running the tool for an app, it'll look if the changes are in the app's
path. When it's the case, the app is detected as impacted and the CI build of
this app should continue. E.g. the changes on `apps/api` should let the `api`
build run.

When the changes are on the path of an app on which the given app depends on,
the CI build of the given app should continue. E.g. the changes on `apps/api`
should let the `frontend` build run as the `frontend` app depends on the `api`
app.

When none of the above cases are encountered, the CI build of the given app
should be skipped.

### Integration

#### Distribution

We could distribute the tool a single binary file. The tool could be downloaded
in a CI build step and be used this way :

```bash
$ ssh -c ssh.yml
```

Detailed usage :

```bash
Usage: ssc -c CONFIG_FILE APP_NAME [PATH]
Skips the CI build of the specified APP_NAME if the changes of this build
aren't impacting this app.

Mandatory arguments :

-c --config[=CONFIG_FILE]   The config file to use (yaml).
APP_NAME                    The app (defined in the config file) for which the
                            current CI build is for.

Optional arguments :

PATH                        The path on which the source code of the project is
                            fetched (defaults to current working dir).
```

As the tool will run the command to stop the current build, it should be
exdcuted by the CI executor itself.

#### Configuration : CI platform

The command to stop the build depends on the CI platform. We can add a
`ci_platform` key to the config file in order to indicate on which platform the
build is executed, so the tool will know which command to run to skip the build,
e.g. :

```yaml
ci_platform: circleci
```

For each platform to support, a dedicated handler should be implemented. For
each platform, the tool should be able to :

- provide the command to execute in order to skip the build
- provide informations related to the changes of the build, such as a PR URL.
These informations may be available in env vars, with specific naming in
function of the platform.

#### Configuration : VCS platform

Similarly to CI platforms, the VCS platform should be specified in order to
let the tool know how to fetch details about the latest changes (e.g. to use
an HTTP API to get the commits of a PR) :

```yaml
vcs_platform: github
```

## Status

In discussion.

## Consequences

Less CI resources usages and shorter build durations.
