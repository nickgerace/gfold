# gfold

[![GitHub](https://img.shields.io/github/license/nickgerace/gfold?style=flat-square)](./LICENSE)
[![Latest SemVer GitHub Tag](https://img.shields.io/github/v/tag/nickgerace/gfold?label=version&style=flat-square)](https://github.com/nickgerace/gfold/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/gfold?style=flat-square)](https://crates.io/crates/gfold)
[![Build Status](https://img.shields.io/github/workflow/status/nickgerace/gfold/merge/main?style=flat-square)](https://github.com/nickgerace/gfold/actions?query=workflow%3Amerge+branch%3Amain)

`gfold` is a CLI-driven application that helps you keep track of multiple Git repositories.

```sh
user at hostname in ~/git
% gfold
great-journey      unclean   main      git@github.com:truth/great-journey.git
installation-zero  bare      main      https://github.com/the-ark/installation-zero.git
sierra             unpushed  dev       https://github.com/forward-unto-dawn/sierra.git
spark              clean     issue343  git@github.com:guilty/spark.git
tartarus           unclean   delta     git@github.com:covenant/tartarus.git
voi                clean     main      https://github.com/earth/voi.git
```

## Description and Motivation

This app displays relevant information for multiple Git repositories in one, or multiple, directories.
While this tool might seem limited in scope and purpose, that is by design.

It prints each repository in alphabetical order, and pads each result based on the longest directory, branch, and status string.
By default, `gfold` looks at every Git repository in the current working directory.
However, if you would like to target another directory, you can pass that path (relative or absolute) as the first argument.

## Should I use `gfold` or `gfld`?

[![Crates.io](https://img.shields.io/crates/v/gfld?style=flat-square)](https://crates.io/crates/gfld)

`gfld` is the new, minimal version of `gfold`.
It contains only one configurable option (an optional, single command-line argument for the target path) and is much smaller than the original application in size.

It is intended for fans of the original application who want a near-configurationaless usability and a smaller footprint on their systems.
It does *not* promise faster runtime performance, but it delivers on the two former goals.

There are two major behavioral differences from the original application: only recursive search is available (similar to `gfold -r`), and all results are combined into one table (inspired by `kubectl get pods -A`).

## Installation

This repository contains two applications: `gfold`, the primary, fully-featured version, and `gfld`, the minimal version.
There is only one recommended method for installing the latter, and the original version has multiple methods for installation.
Thus, this section starts with the minimal version.

**For all installation steps:** it is highly recommended to run `strip` against the binary on compatible systems to reduce executable size.
The following commands were tested on Linux and macOS systems:

```sh
TEMP=$(which gfold) # or replace "gfold" with "gfld"
strip $TEMP
du -h $TEMP | cut -f -1
```

If you do not know where either application was installed, you can use the `which` command on compatible platforms or check your `cargo install` settings.

### Installing `gfld`

Currently, the only recommended method to install `gfld` is by using **[cargo](https://crates.io)** to install the [crate](https://crates.io/crates/gfld).
Fortunately, the minimal application should work on nearly every major platform.

```sh
cargo install gfld
```

> Keeping the crate up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).
>
> ```sh
> cargo install cargo-update
> cargo install-update -a
> ```

### Installing `gfold`

**You can use [macOS Homebrew](https://brew.sh) or [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux)** to install the [tap](https://github.com/nickgerace/homebrew-gfold).

```sh
brew install nickgerace/gfold/gfold
```

**You can use a Linux distribution that supports installing packages from the AUR** to install: [gfold](https://aur.archlinux.org/packages/gfold/) (builds from source) and/or [gfold-git](https://aur.archlinux.org/packages/gfold-git/) (VCS/development package).
Many people choose to use an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers), such as [yay](https://github.com/Jguer/yay) or [paru](https://github.com/Morganamilo/paru), in order to install their AUR packages.

```sh
yay -S gfold
paru -S gfold
```

**You can use [cargo](https://crates.io)** to install the [crate](https://crates.io/crates/gfold) on almost any platform.
Consult the `gfld` section above on how to keep the crate up to date with `cargo-update`.

```sh
cargo install gfold
```

## Usage

For `gfold`: pass in the `-h`, or `--help`, flag to see all the options for using this application.

```sh
gfold
gfold ..
gfold $HOME
gfold /this/is/an/absolute/path
gfold ../../this/is/a/relative/path
gfold ~/path/to/multiple/repositories/ -r
gfold -r $HOME/path/to/multiple/repositories
```

For `gfld`: you can pass in the `-h`, or `--help` too.
However, there is only one method of configuration: an optional, single command-line argument for the target path.
This is a result of the minimal application's design.

```sh
gfld
gfld ..
gfld $HOME
gfld /this/is/an/absolute/path
gfld ../../this/is/a/relative/path
```

## Compatibility

Both applications are intended to be ran on *any* tier one Rust target.
Please [file an issue](https://github.com/nickgerace/gfold/issues) if your platform is unsupported.

## Other Documentation

- **[CHANGELOG.md](./CHANGELOG.md):** follows the [Keep a Changelog](https://keepachangelog.com/) format
- **[DEVELOPING.md](./DEVELOPING.md):** developer tips, tricks, and notes
- **[RELEASE.md](./RELEASE.md):** release process notes

## Code of Conduct

This repository follows and enforces the Rust programming language's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- License: [Apache 2.0](./LICENSE)

## Special Thanks To...

- [@jrcichra](https://github.com/jrcichra) for adding multi-OS support to the original, early-stage CI pipeline
- [@orhun](https://github.com/orhun) for maintaining [all AUR packages](https://github.com/orhun/PKGBUILDs) for `gfold`
- [@yaahc](https://github.com/yaahc) for mentoring during an early refactor
