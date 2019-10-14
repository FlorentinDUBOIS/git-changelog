# Changelog [![Build Status](https://travis-ci.org/FlorentinDUBOIS/changelog.svg?branch=master)](https://travis-ci.org/FlorentinDUBOIS/changelog)

> Generate a changelog using the git commit history

## Getting started

To compile this application, you need a valid rust environment. If you have to
install one, please use [rustup](https://rustup.rs/).

### Installing using cargo

You can install the changelog using the following command:

```sh
$ cargo install --git https://github.com/FlorentinDUBOIS/changelog
```

The binary is placed under `~/.cargo/bin`.

### Compiling from sources

Firstly, get the source code:

```sh
git clone git@github.com:FlorentinDUBOIS/changelog.git
```

Now, compile the changelog:

```sh
cargo build --release
```

You can find the released binary in the `target/release` folder.

## Configuration

An example of the `policy.toml` file used to generate the changelog:

```toml
# Kinds are based on https://github.com/angular/angular/blob/master/CONTRIBUTING.md#type
#
# When a commit match the kind, it will be pushed in the related rubrics.
# If it not match, the commit will not be render on the CHANGELOG.md.
#
# For the changelog project, there are multiples kinds defined:
#
# - build: Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)
# - ci: Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)
# - docs: Documentation only changes
# - feat: A new feature
# - fix: A bug fix
# - perf: A code change that improves performance
# - refactor: A code change that neither fixes a bug nor adds a feature
# - style: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
# - test: Adding missing tests or correcting existing tests
# - infra: A code change related to the infrastructure
# - chore: Task that will be done
[kinds]
build = "Build improvements"
ci = "Continuous integration improvements"
docs = "Documentation enhancements"
feat = "Features"
fix = "Fix changes"
perf = "Performance improvements"
refactor = "Refactor enhancements"
style = "Style changes"
test = "Unit test changes"
infra = "Infrastructure changes"
chore = "Chore tasks"

# Repositories is an array of git repository that will be used in order to render
# the CHANGELOG.md.
[[repositories]]
# Name to give to the repository in the CHANGELOG.md
name = "Changelog"

# Path to the git repository
path = "."

# Scopes are based on https://github.com/angular/angular/blob/master/CONTRIBUTING.md#scope
#
# If scopes are omitted, there is no check on it, so all scopes are accepted.
#
# Scopes are parts of the software which can be impacted during the development.
scopes = [
  "travis",
  "parser",
  "policy",
  "cmd",
  "logger",
  "git",
  "cargo",
  "changelog",
]

# Range allow to select from what and to commit you want to render the CHANGELOG.md.
# The left-hand commit will be hidden and the right-hand commit pushed.
#
# if range is omited all history is used.
#
# example:
#
# range = "820305f..HEAD"

# Link allow to directly retrieve commit details by providing a link pointing to
# them.
#
# Use {hash} to select the place where the commit's hash should be inject
link = "https://github.com/FlorentinDUBOIS/changelog/commit/{hash}"

```

## Usage

In order to get help the flag `--help` will display the help about the current command.

```sh
$ changelog --help
changelog 0.1.0

USAGE:
    changelog [FLAGS] [OPTIONS]

FLAGS:
    -t, --check      Check if the configuration is healthy
    -h, --help       Prints help information
    -v               Increase the log verbosity
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Use the specified configuration file [default: changelog.toml]
    -o, --output <output>    Set the output destination [default: CHANGELOG.md]
```

To generate a `CHANGELOG.md` file, you will need a `changelog.toml` file,
you can find an example in this repository.

```sh
$ changelog -vvvv
Aug 01 10:26:52.844 INFO Skip merge commit, hash: f436d7fadf46e89e7ddc64646220d5834bdb341c, repository: Changelog
Aug 01 10:26:52.896 INFO Skip merge commit, hash: e326eb9920b5f962307df7169a91acb24adaefca, repository: Changelog
Aug 01 10:26:52.953 INFO Skip merge commit, hash: 0550a53fb314ce5c74f68ca7c42aa26953a8b3c0, repository: Changelog
Aug 01 10:26:53.168 WARN Scope is not contained in provided scopes, scope: generate, hash: 2570a9809a60e7c5a8259fc73e7ca41d6566d552, repository: Changelog
Aug 01 10:26:53.213 WARN Scope is not contained in provided scopes, scope: generate, hash: 14ab27767bef9f8eb933f85638545c02bb4bc3aa, repository: Changelog
```

### Tags support

There is nothing special to do in order to make git tags working. There is only one things that you should care. The lightweight git tag are not supported.

To create a git tag, you should use the following command:

```sh
$ git tag -a [-s] vX.Y.Z
```

This will open an editor to add a message on the git tag.