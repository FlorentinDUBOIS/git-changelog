# Git changelog [![Build Status](https://travis-ci.org/FlorentinDUBOIS/changelog.svg?branch=master)](https://travis-ci.org/FlorentinDUBOIS/changelog)

> Generate a changelog using the git commit history

## Getting started

To compile this application, you need an environment with rust and node available. 
If you have to install one of them, you should take a look at [rustup](https://rustup.rs/)
and [nvm](https://github.com/nvm-sh/nvm).

### Compiling from sources

Firstly, we will clone the repository using the following command:

```sh
git clone git@github.com:FlorentinDUBOIS/git-changelog.git
```

Then, we will need to compile the html template using npm:

```shell
npm i && npm run build
```

Finaly, compile the application

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
$ git changelog -h
git-changelog 0.1.0

USAGE:
    git-changelog [FLAGS] [OPTIONS]

FLAGS:
    -t, --check      Check if the configuration is healthy
    -h, --help       Prints help information
    -v               Increase the log verbosity
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Use the specified configuration file [default: changelog.toml]
    -f, --format <format>    Output using the specified format (available formats are: html or markdown) [default: markdown]
    -o, --output <output>    Set the output destination [default: CHANGELOG]

```

To generate a `CHANGELOG.md` file, you will need a `changelog.toml` file,
you can find an example in this repository.

```sh
$ git changelog -vvvvvvv
Jan 04 16:21:57.970 INFO Skip merge commit, hash: 8a42a16
Jan 04 16:21:57.973 INFO Skip merge commit, hash: a48267d
```

### Tags support

There is nothing special to do in order to make git tags working. There is only one things that you should care. The lightweight git tag are not supported.

To create a git tag, you should use the following command:

```sh
git tag -a [-s] vX.Y.Z
```

This will open an editor to add a message on the git tag.