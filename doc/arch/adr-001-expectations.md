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

### Use cases

- on a PR : compare range of commits from <hash> to HEAD
- on a merge : compare the latest commit (eg on the `master` branch or on the
`develop` branch)

CLI usage :

```bash
$ ssc \
    --path apps/api \           # Defaults to cwd when no paths are provided.
    --path apps/frontend \      # Multiple paths can be specified in order to
                                # continue the build for this app when changes
                                # on multiple apps should be considered (i.e.
                                # for integration purposes).
    --pr-url <url> \            # Optional, the URL of a PR to use to fetch a
                                # range of commits.
    --auth <auth> \             # The credentials to use to fetch a range of
                                # commits on the given pr-url.
    --from <from_commit_hash> \ # Defaults to HEAD~1, overriden by the first
                                # PR commit when the --pr-url flag is provided
                                # and not empty, and the PR commits could be
                                # fetched.
    --to <to_commit_hash> \     # Defaults to HEAD.
    --cmd "<skip_job_cmd>"      # The command to use to skip the build.
```

### Workflow

The tool should be used to indicate if the build for a given app should be
skipped.

When the job of the given app should be skipped, the tool will call the command
to skip the job (the command depends on the CI platform). When the job should
not be skipped, the tool will simply exit `0` without calling any commands.

On pull requests (aka merge requests), we can detect the changes contained in
the PR by looking at the commits of the PR.

For commits on the `master` branch, we could only look to the latest commit
to detect the changes.

### Distribution

We could distribute the tool a single binary file. The tool could be downloaded
in a CI build step.

The tool should be ran in a directory which is inside a git repository.

As the tool will run the command to stop the current build, it should be
executed by the CI executor itself.

## Status

In discussion.

## Consequences

Less CI resources usages and shorter build durations.
