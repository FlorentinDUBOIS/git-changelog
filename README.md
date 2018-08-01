# Changelog

> Generate a changelog using the git commit history

## Getting started

To compile this application, you need a valid rust environment. If you have to
install, please use [rustup](https://rustup.rs/).

Firstly, get the source code:

```sh
git clone git@github.com:FlorentinDUBOIS/changelog.git
```

Now, compile the changelog:

```sh
cargo +stable build --release
```

You can find the released binary in the `target/release` folder.

## Usage

In order to get help the flag `--help` will display the help about the current command.

```sh
$ changelog --help

changelog 0.1.0
Florentin DUBOIS <contact@florentin-dubois.fr>

USAGE:
    changelog [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v

SUBCOMMANDS:
    generate    Generate changelog
    help        Prints this message or the help of the given subcommand(s)
    version     Prints version information
```

To generate a `CHANGELOG.md` file, you will need a `policy.toml` file,
you can find an example in this repository.

```sh
$ changelog -vvvvv generate --output CHANGELOG.md --policy policy.toml

Aug 01 10:26:52.844 INFO Skip merge commit, hash: f436d7fadf46e89e7ddc64646220d5834bdb341c, repository: Changelog
Aug 01 10:26:52.896 INFO Skip merge commit, hash: e326eb9920b5f962307df7169a91acb24adaefca, repository: Changelog
Aug 01 10:26:52.953 INFO Skip merge commit, hash: 0550a53fb314ce5c74f68ca7c42aa26953a8b3c0, repository: Changelog
Aug 01 10:26:53.168 WARN Scope is not contained in provided scopes, scope: generate, hash: 2570a9809a60e7c5a8259fc73e7ca41d6566d552, repository: Changelog
Aug 01 10:26:53.213 WARN Scope is not contained in provided scopes, scope: generate, hash: 14ab27767bef9f8eb933f85638545c02bb4bc3aa, repository: Changelog
```
