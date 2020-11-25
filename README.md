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

There are multiple ways to install `gfold`, but here are some recommended methods...

Installation Methods | `linux-gnu-amd64` | `macos-amd64` | `windows-amd64`
--- | --- | --- | --
Homebrew | x | x | -
Arch User Repository (AUR) | x | - | -
Cargo Install | x | x | x
GitHub Release Binary | x | x | x

### Homebrew

You can use [Homebrew](https://brew.sh) to install the [tap](https://github.com/nickgerace/homebrew-gfold) for `gfold`.

```bash
brew install nickgerace/gfold/gfold
```

Alternatively, you can do...

```bash
brew tap nickgerace/gfold
brew install gfold
```

Running `brew help` or `man brew` can help you use `brew` locally.
You can check out [Homebrew's documentation](https://docs.brew.sh) as well.

### Arch User Repository (AUR)

This application is available for all Linux distributions that support installing packages from the AUR.

- [gfold](https://aur.archlinux.org/packages/gfold/) (builds from source)
- [gfold-bin](https://aur.archlinux.org/packages/gfold-bin/) (uses the GitHub release binary)
- [gfold-git](https://aur.archlinux.org/packages/gfold-git/) (VCS/development package)

Many people choose to use an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers), such as [yay](https://github.com/Jguer/yay) (example: `yay -S gfold`), in order to install their AUR packages.

### Cargo Install

You can install from [crates.io](https://crates.io/crates/gfold) by executing...

```bash
cargo install gfold
```

### GitHub Release Binary

You can obtain `gfold` via the [latest GitHub release](https://github.com/nickgerace/gfold/releases/latest).
Once you have it downloaded, you can add it to your `PATH`.
Here is an example on how to do that on macOS and Linux...

```bash
chmod +x gfold
mv gfold /usr/local/bin/
```

You may have to reload your shell in order to see `gfold` in your `PATH`.

#### Advanced Management

You can use symbolic links to swap between versions, and manage multiple at a time.
Here is a full install workflow example...

```bash
wget https://github.com/nickgerace/gfold/releases/download/$VERSION/gfold-$PLATFORM
mv gfold-$PLATFORM gfold-$VERSION
chmod +x gfold-$VERSION

mkdir /usr/local/gfold/
mv gfold-$VERSION /usr/local/gfold/
ln -s /usr/local/gfold/gfold-$VERSION /usr/local/bin/gfold
```

Now, you can add/remove versions of the binary from `/usr/local/gfold/`, and change the symbolic link as needed.

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
- **macOS**: `macos-amd64`
- **Windows 10**: `windows-amd64`

## Issues, Pull Requests, and Contributing

Please follow the issue template when filing an issue.
If making a pull request, the requirements for merging are...

1. An issue must be linked, whether it is a new one or an existing one.
2. The pull request branch needs to be rebased with `nickgerace/gfold` on `main`.
3. Pull request commits need to be squashed (there may be exceptions to this on a case-by-case basis).

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
