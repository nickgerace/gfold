# gfold

[![tag](https://img.shields.io/github/v/tag/nickgerace/gfold?sort=semver&logo=github&label=version&style=flat-square&color=blue)](https://github.com/nickgerace/gfold/releases/latest)
[![crates.io](https://img.shields.io/crates/v/gfold?style=flat-square&logo=rust&color=orange)](https://crates.io/crates/gfold)
[![build](https://img.shields.io/github/workflow/status/nickgerace/gfold/merge/main?style=flat-square)](https://github.com/nickgerace/gfold/actions?query=workflow%3Amerge+branch%3Amain)
[![license](https://img.shields.io/github/license/nickgerace/gfold?style=flat-square&color=purple)](./LICENSE)

`gfold` is a CLI-driven application that helps you keep track of multiple Git repositories.

```
% gfold
astrid  unclean   main       git@github.com:db/astrid.git
fev     bare      main       https://github.com/institute/fev.git
gb      unpushed  dev        https://github.com/hrothgar/gb.git
neloth  unclean   patch      git@github.com:telvanni/neloth.git
pam     clean     main       https://github.com/onc/pam.git
prime   clean     issue2287  git@github.com:bos/prime.git
```

## Description

This app displays relevant information for multiple Git repositories in one to many directories.
While this tool might seem limited in scope and purpose, that is by design.

It prints each repository in alphabetical order, and pads each result based on the longest directory, branch, and status string.
By default, `gfold` looks at every Git repository via traversal from the current working directory.
However, if you would like to target another directory, you can pass that path (relative or absolute) as the first argument.

## Installation

There are multiple methods for installing `gfold`.

### Homebrew (macOS only)

You can use [Homebrew](https://brew.sh) to install the [tap](https://github.com/nickgerace/homebrew-nickgerace/blob/main/Formula/gfold.rb).

```bash
brew install nickgerace/nickgerace/gfold
```

The original [tap](https://github.com/nickgerace/homebrew-gfold) will no longer be maintained after version `2.0.1`.
Please migrate to the new tap using the command above.
Neither the current and deprecated taps will work with [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux).

### AUR

You can use a Linux distribution that supports installing packages from the AUR, [Arch User Respository](https://aur.archlinux.org/), to install the following:

- [**gfold**](https://aur.archlinux.org/packages/gfold/): builds from source
- [**gfold-git**](https://aur.archlinux.org/packages/gfold-git/): development package

Many people choose to use an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers), such as [paru](https://github.com/Morganamilo/paru), in order to install their AUR packages.

```bash
paru -S gfold
```

### Cargo (recommended)

You can use [cargo](https://crates.io) to install the [crate](https://crates.io/crates/gfold) on almost any platform.

```bash
cargo install --locked gfold
```

Keeping the crate up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).

```bash
cargo install --locked cargo-update
cargo install-update -a
```

### Binary from a Release

If you do not want to use one of the above installation methods, you can download a binary from the [releases](https://github.com/nickgerace/gfold/releases) page.

```bash
curl https://raw.githubusercontent.com/nickgerace/gfold/main/scripts/install.sh | sh
```

For security, please note that the installation convenience script does not verify the binary with a checksum.
Discretion is advised, including downloading and reading the script before execution.

To uninstall `gfold` fully, after using this installation method, execute the following script:

```bash
curl https://raw.githubusercontent.com/nickgerace/gfold/main/scripts/uninstall.sh | sh
```

## Usage

Pass in the `-h`, or `--help`, flag to see all the options for using this application.

```bash
gfold
gfold ..
gfold $HOME
gfold ~/
gfold /this/is/an/absolute/path
gfold ../../this/is/a/relative/path
```

## Compatibility

`gfold` is intended to be ran on *any* tier one Rust 🦀 target that `git` is also available on.
Please [file an issue](https://github.com/nickgerace/gfold/issues) if your platform is unsupported.

## Troubleshooting

If `fold` from GNU Coreutils is installed on macOS via `brew`, it will be named `gfold`.
You can avoid this collision with shell aliases, shell functions, and/or `PATH` changes.
Here is an example with the `o` dropped from `gfold`:

```bash
alias gfld=$HOME/.cargo/bin/gfold
```
