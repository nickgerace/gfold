# gfold

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fnickgerace%2Fgfold%2Fbadge&style=flat)](https://actions-badge.atrox.dev/nickgerace/gfold/goto)

```gfold``` is a CLI application that helps you keep track of multiple Git repositories.

```bash
nick at hostname in ~/git
% gfold
bat         clean    master  git@github.com:sharkdp/bat.git
exa         clean    master  git@github.com:ogham/exa.git
gfold       unclean  master  git@github.com:nickgerace/gfold.git
nushell     clean    master  git@github.com:nushell/nushell.git
tockilator  clean    master  git@github.com:oxidecomputer/tockilator.git
```

## Description and Motivation

This app displays relevant information for multiple Git repositories in one directory or folder.
It prints each repository in alphabetical order, and pads each result based on the longest directory name.

By default, ```gfold``` looks at every Git repository in the current working directory.
However, you can use the ```-p/--path``` flag to target another directory.

While this tool might seem limited in scope and purpose, that is by design.
Features, such as recursive search and async-await support, are future goals.
This application aims to do one or few things well.

## Installation

You can download a [release](https://github.com/nickgerace/gfold/releases), or you can build and install ```gfold``` by executing the following.

```bash
cargo install --git https://github.com/nickgerace/gfold
```

## Usage

There's only two usage options at the moment (CWD or specified path), but you can use the ```--help``` flag for more details.

```bash
gfold --help
```

Here are some example invocations...

```bash
gfold
gfold -p ..
gfold -p $HOME
```

## Compatibility

```gfold``` should work on Linux amd64, macOS amd64, and Windows amd64.
All external crates were vetted for multi-platform (including Windows 10) support.

## Future Plans

- Add recursive function to search sub-directories.
- Replace sequential functions with async-await.
- Add better error handling for edge cases (i.e. bare repositories).
- Add version checking, using the GitHub API ([example: bat](https://api.github.com/repos/sharkdp/bat/releases/latest)), to compare the latest tag with Clap's local version.

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- Contributors: [graph](https://github.com/nickgerace/gfold/graphs/contributors)
- License: MIT
