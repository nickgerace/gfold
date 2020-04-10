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

One more thing: doing one or few things well includes not only the usual Linux/macOS amd64 users, but users from multiple major platforms and architectures.

## Installation

While there are no release builds at the moment, you can build ```gfold``` by executing the following.

```bash
cargo build --release
```

## Usage

There's only two usage options at the moment (CWD or specified path), but you can use the ```--help``` flag for more details.

```bash
gfold --help
```

## Compatibility

```gfold``` should work on Linux, macOS, and Windows.
All external crates were vetted for multi-platform (including Windows 10) support.

The CI/CD pipeline currently only tests Ubuntu amd64 builds, but this is subject to change.
While not officially tested in a GitHub Action, its worth noting that the application is developed natively on a Windows 10 amd64 machine.

## Future Plans

- Add recursive function to search sub-directories.
- Replace sequential functions with async-await.
- Add installation instructions with releases available.
- Add version checking, using the GitHub API ([example: bat](https://api.github.com/repos/sharkdp/bat/releases/latest)), to compare the latest tag with Clap's local version.

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- License: MIT
