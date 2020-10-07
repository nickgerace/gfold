# gfold

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache2.0-yellow.svg)](https://opensource.org/licenses/apache2.0)
[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fnickgerace%2Fgfold%2Fbadge&style=flat)](https://actions-badge.atrox.dev/nickgerace/gfold/goto)

```gfold``` is a CLI application that helps you keep track of multiple Git repositories.

```bash
user at hostname in ~/git
% gfold
great-journey      unclean  main      git@github.com:truth/great-journey.git
installation-zero  bare     main      https://github.com/the-ark/installation-zero.git
sierra             clean    dev       https://github.com/forward-unto-dawn/sierra.git
spark              clean    issue343  git@github.com:guilty/spark.git
tartarus           unclean  delta     git@github.com:covenant/tartarus.git
voi                clean    main      https://github.com/earth/voi.git
```

## Description and Motivation

This app displays relevant information for multiple Git repositories in one, or multiple, directories.
While this tool might seem limited in scope and purpose, that is by design.

It prints each repository in alphabetical order, and pads each result based on the longest directory, branch, and status string.
By default, ```gfold``` looks at every Git repository in the current working directory.
However, if you would like to target another directory, you can pass that path (relative or absolute) as the first argument.

## Installation

There are multiple ways to install ```gfold```, but here are the recommended methods.

### AUR (Arch User Repository)

This application is available for all Linux distributions that support installing packages from the AUR.
Special thanks to [orhun](https://github.com/orhun) for [maintaining](https://github.com/orhun/PKGBUILDs) these packages.

- [gfold](https://aur.archlinux.org/packages/gfold/) (builds from source)
- [gfold-git](https://aur.archlinux.org/packages/gfold-git/) (VCS/development package)

**Note**: many people choose to use an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers), such as [yay](https://github.com/Jguer/yay) (example: ```yay -S gfold```), in order to install their AUR packages.

### Cargo Install

You can build from source with ```cargo``` by executing the following...

```bash
cargo install --git https://github.com/nickgerace/gfold --tag 0.5.0
```

## Usage

For all the ways on how to use this application, pass in the ```--help``` flag.

```bash
gfold --help
```

Here are some example invocations...

```bash
gfold
gfold ..
gfold $HOME
gfold /this/is/an/absolute/path
gfold ../../this/is/a/relative/path
gfold ~/repositories/ -r
gfold -r $HOME/repositories
```

## Compatibility

All external crates were vetted for support on all three major desktop platforms.
```gfold``` is tested for the latest version of the following systems, but may work on more...

- **Linux**: ```linux-gnu-amd64```
- **macOS**: ```darwin-amd64```
- **Windows 10**: ```windows-amd64```

## Changelog

Please check out [CHANGELOG.md](https://github.com/nickgerace/gfold/blob/master/CHANGELOG.md) for more information.
It follows the [Keep a Changelog](https://keepachangelog.com/) format.

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- License: [Apache 2.0](https://github.com/nickgerace/gfold/blob/master/LICENSE)

## Special Thanks

- [@yaahc](https://github.com/yaahc) (mentoring)
- [@orhun](https://github.com/orhun) (maintaining AUR packages)
- [@jrcichra](https://github.com/jrcichra) (adding multi-OS support to the original CI pipeline)
