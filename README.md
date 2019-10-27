# should-skip-ci

A tool to use to skip a CI build when the changes of a range of commits aren't
related to the given path(s).

See the [expectations](/doc/arch/adr-001-expectations.md) to know why this tool
has been created.

## Development

Open a shell into the `rust` container :

```bash
$ make shell
```

and then use `cargo` to launch the tool :

```bash
$ cargo run -- <ssc_options>
```
