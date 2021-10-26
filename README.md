# gfold

[![tag](https://img.shields.io/github/v/tag/nickgerace/gfold?sort=semver&logo=github&label=version&style=flat-square&color=blue)](https://github.com/nickgerace/gfold/releases/latest)
[![crates.io](https://img.shields.io/crates/v/gfold?style=flat-square&logo=rust&color=orange)](https://crates.io/crates/gfold)
[![build](https://img.shields.io/github/workflow/status/nickgerace/gfold/merge/main?style=flat-square)](https://github.com/nickgerace/gfold/actions?query=workflow%3Amerge+branch%3Amain)
[![license](https://img.shields.io/github/license/nickgerace/gfold?style=flat-square&color=purple)](./LICENSE)

`gfold` is a CLI-driven application that helps you keep track of multiple Git repositories.

```sh
% gfold
astrid  unclean   main       git@github.com:db/astrid.git
fev     bare      main       https://github.com/institute/fev.git
gb      unpushed  dev        https://github.com/hrothgar/gb.git
neloth  unclean   patch      git@github.com:telvanni/neloth.git
pam     clean     main       https://github.com/onc/pam.git
prime   clean     issue2287  git@github.com:bos/prime.git
```

## Description and Motivation

This app displays relevant information for multiple Git repositories in one to many directories.
While this tool might seem limited in scope and purpose, that is by design.

It prints each repository in alphabetical order, and pads each result based on the longest directory, branch, and status string.
By default, `gfold` looks at every Git repository via traversal from the current working directory.
However, if you would like to target another directory, you can pass that path (relative or absolute) as the first argument.

## Installation

There are multiple methods for installing `gfold`.

### Homebrew (macOS only)

You can use [Homebrew](https://brew.sh) to install the [tap](https://github.com/nickgerace/homebrew-nickgerace/blob/main/Formula/gfold.rb).

```sh
brew install nickgerace/nickgerace/gfold
```

_Notes:_
- _The original [tap](https://github.com/nickgerace/homebrew-gfold) is still actively maintained, but is deprecated. Please migrate to the new tap._
- _Both the current and deprecated taps may not work with [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux)._

### AUR

You can use a Linux distribution that supports installing packages from the AUR, [Arch User Respository](https://aur.archlinux.org/), to install the following:

- [gfold](https://aur.archlinux.org/packages/gfold/) - builds from source
- [gfold-git](https://aur.archlinux.org/packages/gfold-git/) - development package

Many people choose to use an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers), such as [yay](https://github.com/Jguer/yay) or [paru](https://github.com/Morganamilo/paru), in order to install their AUR packages.

```sh
yay -S gfold
paru -S gfold
```

### Cargo (recommended)

You can use [cargo](https://crates.io) to install the [crate](https://crates.io/crates/gfold) on almost any platform.

```sh
cargo install gfold
```

Keeping the crate up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).

```sh
cargo install cargo-update
cargo install-update -a
```

### Binary from a Release

If you do not want to use one of the above installation methods, you can download a binary from the [releases](https://github.com/nickgerace/gfold/releases) page.
The following convenience script can be used on macOS and Linux amd64 systems (requires `wget`, `jq`, and `curl` to be installed):

```sh
(
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    if [ "$OS" = "linux" ]; then OS=linux-gnu; fi
    LATEST=$(curl -s https://api.github.com/repos/nickgerace/gfold/releases/latest | jq -r ".tag_name")
    wget -O gfold https://github.com/nickgerace/gfold/releases/download/$LATEST/gfold-$OS-amd64
    chmod +x gfold
    sudo mv gfold /usr/local/bin/gfold
)
```

_Note: the above convenience script does not verify the binary with a checksum.
Discretion is advised._

## Usage

Pass in the `-h`, or `--help`, flag to see all the options for using this application.

```sh
gfold
gfold ..
gfold $HOME
gfold ~/
gfold /this/is/an/absolute/path
gfold ../../this/is/a/relative/path
```

## Compatibility

`gfold` is intended to be ran on *any* tier one Rust target.
Please [file an issue](https://github.com/nickgerace/gfold/issues) if your platform is unsupported.

## Troubleshooting

If `fold` from GNU Coreutils is installed on macOS via `brew`, it will be named `gfold`.
You can avoid this collision with shell aliases, shell functions, and/or `PATH` changes.
Here is an example with the `o` dropped from `gfold`:

```sh
alias gfld=$HOME/.cargo/bin/gfold
```
