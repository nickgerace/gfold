# gfold

[![GitHub](https://img.shields.io/github/license/nickgerace/gfold?style=flat-square)](./LICENSE)
[![Latest SemVer GitHub Tag](https://img.shields.io/github/v/tag/nickgerace/gfold?label=version&style=flat-square)](https://github.com/nickgerace/gfold/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/gfold?style=flat-square)](https://crates.io/crates/gfold)
[![Build Status](https://img.shields.io/github/workflow/status/nickgerace/gfold/merge/main?style=flat-square)](https://github.com/nickgerace/gfold/actions?query=workflow%3Amerge+branch%3Amain)

`gfold` is a CLI application that helps you keep track of multiple Git repositories.

```bash
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

## Installation

You can use **[macOS Homebrew](https://brew.sh)** or **[Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux)** to install the [tap](https://github.com/nickgerace/homebrew-gfold).

```bash
brew install nickgerace/gfold/gfold
```

If using a **Linux distribution that supports installing packages from the AUR**, you can install from three packages: [gfold](https://aur.archlinux.org/packages/gfold/) (builds from source), [gfold-bin](https://aur.archlinux.org/packages/gfold-bin/) (uses the GitHub release binary), and [gfold-git](https://aur.archlinux.org/packages/gfold-git/) (VCS/development package).
Many people choose to use an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers), such as [yay](https://github.com/Jguer/yay) or [paru](https://github.com/Morganamilo/paru), in order to install their AUR packages.

```bash
yay -S gfold
paru -S gfold
```

You can install the [crate](https://crates.io/crates/gfold) on any platform with **[cargo](https://crates.io)**.

```bash
cargo install gfold
```

Keeping the crate up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).

```sh
cargo install cargo-update
cargo install-update -a
```

You can obtain `gfold` via the **[latest GitHub release](https://github.com/nickgerace/gfold/releases/latest)**.
Once you have it downloaded, you can add it to your `PATH`.
You may have to reload your shell in order to see `gfold` in your `PATH`.

```bash
chmod +x gfold
mv gfold /usr/local/bin/
```

You can use symbolic links to swap between versions, and manage multiple at a time.
With this workflow, you can add/remove versions of the binary from `/usr/local/gfold/`, and change the symbolic link as needed.

```bash
wget https://github.com/nickgerace/gfold/releases/download/$VERSION/gfold-$PLATFORM
mv gfold-$PLATFORM gfold-$VERSION
chmod +x gfold-$VERSION

mkdir /usr/local/gfold/
mv gfold-$VERSION /usr/local/gfold/
ln -s /usr/local/gfold/gfold-$VERSION /usr/local/bin/gfold
```

## Usage

For all the ways on how to use this application, pass in the `-h`, or `--help`, flag.

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
gfold ~/path/to/multiple/repositories/ -r
gfold -r $HOME/path/to/multiple/repositories
```

## Compatibility

`gfold`, and its external crates, support all three major desktop platforms.
It is tested for the latest versions of the following systems, but may work on more...

- **Linux**: `linux-gnu-amd64`
- **macOS**: `darwin-amd64`
- **Windows 10**: `windows-amd64`

## Changelog

Please check out [CHANGELOG.md](./CHANGELOG.md) for more information.
It follows the [Keep a Changelog](https://keepachangelog.com/) format.

## Code of Conduct

This repository follows and enforces the Rust programming language's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- License: [Apache 2.0](./LICENSE)

## Special Thanks To...

- [@jrcichra](https://github.com/jrcichra) for adding multi-OS support to the original CI pipeline
- [@orhun](https://github.com/orhun) for maintaining [all three AUR packages](https://github.com/orhun/PKGBUILDs)
- [@yaahc](https://github.com/yaahc) for mentoring
