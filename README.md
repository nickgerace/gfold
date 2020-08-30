# gfold

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache2.0-yellow.svg)](https://opensource.org/licenses/apache2.0)
[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fnickgerace%2Fgfold%2Fbadge&style=flat)](https://actions-badge.atrox.dev/nickgerace/gfold/goto)

```gfold``` is a CLI application that helps you keep track of multiple Git repositories.

```bash
user at hostname in ~/git
% gfold
bat         clean    master  git@github.com:sharkdp/bat.git
bare-repo   bare     dev     https://github.com/<user>/bare-repo.git
exa         clean    master  git@github.com:ogham/exa.git
gfold       unclean  async   git@github.com:nickgerace/gfold.git
nushell     clean    master  https://github.com/nushell/nushell.git
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

There are multiple ways to install ```gfold```, but here are the recommended methods.

### AUR (Arch User Repository)

This application is available for all Linux distributions that support installing packages from the AUR.
Special thanks to [orhun](https://github.com/orhun) for maintaining these packages.

- [gfold](https://aur.archlinux.org/packages/gfold/) (builds from source)
- [gfold-git](https://aur.archlinux.org/packages/gfold-git/) (VCS/development package)

Note: many folks chose to use an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers), such as [yay](https://github.com/Jguer/yay) (e.g: yay -S gfold), in order to install their AUR packages.

### Cargo Install

You can build from source with ```cargo``` by executing the following...

```bash
cargo install --git https://github.com/nickgerace/gfold
```

### Other

There may be some [releases](https://github.com/nickgerace/gfold/releases) available, but there is not a consistent, CI/CD pipeline for this tool yet.

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

All external crates were vetted for multi-platform (including Windows 10) support.
```gfold``` is tested for the following systems, but may work on more...

- Linux amd64 (default, dynamically linked)
- Linux amd64 (MUSL, statically linked)
- macOS amd64
- Windows 10 amd64

## Future Plans

- Add recursive function to search sub-directories.
- Replace sequential functions with async-await.
- Add version checking, using the GitHub API ([example: bat](https://api.github.com/repos/sharkdp/bat/releases/latest)), to compare the latest tag with the CLI's local version string.
- Create a consistent [CHANGELOG.md](https://keepachangelog.com/).

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- License: [Apache 2.0](https://github.com/nickgerace/gfold/blob/master/LICENSE)

## Special Thanks

- [yaahc](https://github.com/yaahc) (mentoring)
- [orhun](https://github.com/orhun) (maintaining AUR packages)
- [jrcichra](https://github.com/jrcichra) (adding multi-OS support to the original CI pipeline)
