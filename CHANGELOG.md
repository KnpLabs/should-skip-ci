# Changelog

## v0.2.3 (August 22, 2024)

This is a maintenance release providing the latest (1.80.1) rust environment,
and projects' dependencies updates ([#38](https://github.com/KnpLabs/should-skip-ci/pull/38)).

The compiled binary size has been slightly reduced due to dependencies
simplification, thus you are encouraged to update ssc to this version.


## v0.2.2 (November 07, 2022)

This is a fix release with documentation updates. Also, the rust version and
packages has been upgraded to the latest version ([#35](https://github.com/KnpLabs/should-skip-ci/pull/35)).

### Documentation updates

- indicate that any git node is a valid `path` arg ([#33](https://github.com/KnpLabs/should-skip-ci/pull/33))
- precise limitations ([#34](https://github.com/KnpLabs/should-skip-ci/pull/34))
- added the maintainers list ([#32](https://github.com/KnpLabs/should-skip-ci/pull/32))

## v0.2.1 (March 09, 2022)

This is mainly a maintenance release, with however some highlights such as the
distributed binary size reduction, and the update to the latest rust version and
edition (also the packages were updated too).
The project structure has also been updated to be more convenient to work with.

The usage of the tool is the same than with the v0.2.0.
You're encouraged to update to this new version to save some Internet bandwidth
and disk space during installation.

### Noticeable changes

- Optimize binary size ([#24](https://github.com/KnpLabs/should-skip-ci/pull/24)) :
reduces the size of the distributed binary from 3.2M down to 567K.
Also update rust and used packages to the latest version.
- Bump rust edition to 2021 and restructure the project sources ([#27](https://github.com/KnpLabs/should-skip-ci/pull/27)).

### Minor fixes

- Fix README typo ([#23](https://github.com/KnpLabs/should-skip-ci/pull/23)).
- Fix circleci deprecation ([#26](https://github.com/KnpLabs/should-skip-ci/pull/26)).
- Fix cargo warning ([#28](https://github.com/KnpLabs/should-skip-ci/pull/28)).
- Update documentation about circleci example ([#29](https://github.com/KnpLabs/should-skip-ci/pull/29)).

## v0.2.0 (May 14, 2020)

Feature/add logger [#20](https://github.com/KnpLabs/should-skip-ci/pull/20).
Adds `-v` and `-vv` optional flags to increase verbosity
(`-v` for info, prints the detected diff ; `-vv` for debug, prints the ran git
commands).

## v0.1.0 (January 08, 2020)

First release.

SSC can be used to identify the changes on given paths for these use cases :

- compare a range of commits on a branch
- compare the changes on a merge commit on the base branch

See the README.md file of this tag for full details.
