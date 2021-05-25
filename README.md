# gfold

[![tag](https://img.shields.io/github/v/tag/nickgerace/gfold?label=version&style=flat-square)](https://github.com/nickgerace/gfold/releases/latest)
[![crates.io](https://img.shields.io/crates/v/gfold?style=flat-square)](https://crates.io/crates/gfold)
[![docs.rs](https://img.shields.io/docsrs/gfold?style=flat-square)](https://docs.rs/gfold)
[![build](https://img.shields.io/github/workflow/status/nickgerace/gfold/merge/main?style=flat-square)](https://github.com/nickgerace/gfold/actions?query=workflow%3Amerge+branch%3Amain)
[![license](https://img.shields.io/github/license/nickgerace/gfold?style=flat-square)](./LICENSE)

`gfold` is a CLI-driven application that helps you keep track of multiple Git repositories.

```sh
% gfold
astrid         unclean   main       git@github.com:db/astrid.git
fev            bare      main       https://github.com/institute/fev.git
gb             unpushed  dev        https://github.com/hrothgar/gb.git
liberty-prime  clean     issue2287  git@github.com:bos/liberty-prime.git
neloth         unclean   patch      git@github.com:telvanni/neloth.git
synth          clean     main       https://github.com/railroad/synth.git
```

## Description and Motivation

This app displays relevant information for multiple Git repositories in one, or multiple, directories.
While this tool might seem limited in scope and purpose, that is by design.

It prints each repository in alphabetical order, and pads each result based on the longest directory, branch, and status string.
By default, `gfold` looks at every Git repository via traversal from the current working directory.
However, if you would like to target another directory, you can pass that path (relative or absolute) as the first argument.

## Installation

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

```sh
cargo install gfold
```

> Keeping the crate up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).
>
> ```sh
> cargo install cargo-update
> cargo install-update -a
> ```

### Post-Installation

It is highly recommended to run `strip` against the binary on compatible systems to reduce executable size.

```sh
TEMP=$(command -v gfold) && du -h $TEMP && strip $TEMP && du -h $TEMP
```

> The above script will exit with a non-zero exit code if `gfold` is not installed and/or is not in your `PATH`.

## Usage

Pass in the `-h`, or `--help`, flag to see all the options for using this application.

```sh
gfold
gfold ..
gfold $HOME
gfold /this/is/an/absolute/path
gfold ../../this/is/a/relative/path
```

## Compatibility

`gfold` is intended to be ran on *any* tier one Rust target.
Please [file an issue](https://github.com/nickgerace/gfold/issues) if your platform is unsupported.

## Where is `gfld`?

`gfld` was an experimental, minimal version of `gfold`.
It was intended to potentially replace `gfold`, but has since been removed.
All optimizations and lessons learned from the project have since been integrated into `gfold`.
Check out the [removal issue to learn more](https://github.com/nickgerace/gfold/issues/110).

## Other Documentation

- **[CHANGELOG.md](./CHANGELOG.md):** follows the [Keep a Changelog](https://keepachangelog.com/) format
- **[DEVELOPING.md](./DEVELOPING.md):** developer tips, tricks, and notes
- **[RELEASE.md](./RELEASE.md):** release process notes

## Code of Conduct

This repository follows and enforces the Rust programming language's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).


## Maintainers

- [@nickgerace](https://nickgerace.dev)

## Special Thanks

- [@jrcichra](https://github.com/jrcichra) for adding multi-OS support to the original, early-stage CI pipeline
- [@orhun](https://github.com/orhun) for maintaining [all AUR packages](https://github.com/orhun/PKGBUILDs)
- [@yaahc](https://github.com/yaahc) for mentoring during an early refactor
