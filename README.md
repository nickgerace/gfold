# gfold

[![latest release tag](https://img.shields.io/github/v/tag/nickgerace/gfold?sort=semver&logo=git&logoColor=white&label=version&style=flat-square&color=blue)](https://github.com/nickgerace/gfold/releases/latest)
[![crates.io version](https://img.shields.io/crates/v/gfold?style=flat-square&logo=rust&color=orange)](https://crates.io/crates/gfold)
[![license](https://img.shields.io/github/license/nickgerace/gfold?style=flat-square&logo=apache&color=silver)](./LICENSE)
[![build status](https://img.shields.io/github/actions/workflow/status/nickgerace/gfold/ci.yml?branch=main&style=flat-square&logo=github&logoColor=white)](https://github.com/nickgerace/gfold/actions)

`gfold` is a CLI-driven application that helps you keep track of multiple Git repositories.

[![A GIF showcasing gfold in action](https://raw.githubusercontent.com/nickgerace/gfold/main/assets/demo.gif)](https://raw.githubusercontent.com/nickgerace/gfold/main/assets/demo.gif)

If you'd prefer to use the classic display mode by default, and avoid setting the flag every time, you can set it in the config file (see **Usage** section).

## Description

This app displays relevant information for multiple Git repositories in one to many directories.
While this tool might seem limited in scope and purpose, that is by design.

By default, `gfold` looks at every Git repository via traversal from the current working directory.
If you would like to target another directory, you can pass its path (relative or absolute) as the first argument or change the default path in the config file.

After traversal, `gfold` leverages [rayon](https://github.com/rayon-rs/rayon) to perform concurrent, read-only analysis of all Git repositories detected.
Analysis is performed by leveraging the [git2-rs](https://github.com/rust-lang/git2-rs) library.

## Usage

Pass in `--help` flag to see all the options for using this application.

```shell
gfold
gfold ..
gfold $HOME
gfold ~/
gfold /this/is/an/absolute/path
gfold ../../this/is/a/relative/path
```

### Config File

Upon execution, `gfold` will look for a config file at the following path on macOS, Linux and similar operating systems:

```shell
$HOME/.config/gfold.toml
```

On Windows, the lookup path will be in a similar location.

```powershell
{FOLDERID_Profile}\.config\gfold.toml
```

Creating and using the config file is entirely optional.

For config file creation, you can use the `--dry-run` flag to print valid TOML.
Here is an example config file creation workflow on macOS, Linux and similar platforms:

```shell
gfold -d classic -c never ~/ --dry-run > $HOME/.config/gfold.toml
```

Here are the contents of the resulting config file:

```toml
path = '/home/neloth'
display_mode = 'Classic'
color_mode = 'Never'
```

Let's say you created a config file, but wanted to execute `gfold` with entirely different settings _and_ you want to ensure that
you do not accidentally inherit options from the config file.
In that scenario you can ignore your config file by using the `-i` flag.

```shell
gfold -i
```

You can restore the config file to its defaults by using the same flag.

```shell
gfold -i > $HOME/.config/gfold.toml
```

In addition, you can ignore the existing config file, configure specific options, and use defaults for unspecified options all at once.
Here is an example where we want to use the classic display mode and override all other settings with their default values:

```shell
gfold -i -d classic > $HOME/.config/gfold.toml
```

You can back up a config file and track its history with `git`.
On macOS, Linux, and most systems, you can link the file back to a `git` repository.

```shell
ln -s path/to/repository/gfold.toml $HOME/.config/gfold.toml
```

Now, you can update the config file within your repository and include the linking as part of your environment setup workflow.

## Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/gfold.svg)](https://repology.org/project/gfold/versions)

### Homebrew Install (macOS only)

You can use [Homebrew](https://brew.sh) to install the [tap](https://github.com/nickgerace/homebrew-nickgerace/blob/main/Formula/gfold.rb).

```shell
brew install nickgerace/nickgerace/gfold
```

_Note:_ the tap may not work with [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux).

### Arch Linux

[![arch linux](https://img.shields.io/archlinux/v/community/x86_64/gfold?logo=archlinux&logoColor=white&style=flat-square&color=blue)](https://archlinux.org/packages/community/x86_64/gfold/)

You can use [pacman](https://wiki.archlinux.org/title/Pacman) to install `gfold` from the [community repository](https://archlinux.org/packages/community/x86_64/gfold/).

```shell
pacman -S gfold
```

### Nix and NixOS

You can install `gfold` from [nixpkgs](https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/version-management/gfold/default.nix):

```shell
nix-env --install gfold
```

If you are using [flakes](https://nixos.wiki/wiki/Flakes), you can install using the `nix` command directly.

```shell
nix profile install "nixpkgs#gfold"
```

### Cargo Install

You can use [cargo](https://crates.io) to install the [crate](https://crates.io/crates/gfold) on almost any platform.

```shell
cargo install gfold
```

Use the `--locked` flag if you'd like Cargo to use `Cargo.lock`.

```shell
cargo install --locked gfold
```

Keeping the crate up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).

```shell
cargo install cargo-update
cargo install-update -a
```

### Download a Binary

If you do not want to use one of the above installation methods and do not want to clone the repository, you can download a binary from the [releases](https://github.com/nickgerace/gfold/releases) page.
For an example on how to do that, refer to the [manual install](./docs/MANUAL_INSTALL.md) guide.

### Build From Source

If you would like an example on how to build from source, refer to the [manual install](./docs/MANUAL_INSTALL.md) guide.

### Preferred Installation Method Not Listed?

Please [file an issue](https://github.com/nickgerace/gfold/issues/new)!

## Compatibility

`gfold` is intended to be ran on _any_ tier one Rust 🦀 target.
Please [file an issue](https://github.com/nickgerace/gfold/issues) if your platform is unsupported.

## Troubleshooting and Known Issues

If you encounter unexpected behavior or a bug and would like to see more details, please run `gfold` with the following
environment variables:

```shell
# You may also want to add relevant arg(s) and flag(s).
RUST_BACKTRACE=1 RUST_LOG=debug gfold
```

If the issue persists, please [file an issue](https://github.com/nickgerace/gfold/issues).

### Tuning Environment Variables

Since [`RUST_BACKTRACE`](https://doc.rust-lang.org/std/backtrace/index.html) and
[`RUST_LOG`](https://docs.rs/env_logger/latest/env_logger/), do not have `gfold`-specific behaviors, you can adjust
them just as you would in other projects to aid investigation.
Please attach relevant logs from execution with _sensitive bits redacted_ in order to help resolve your issue.

### Coreutils Collision on macOS

If `fold` from [GNU Coreutils](https://www.gnu.org/software/coreutils/) is installed on macOS via `brew`, it will be named `gfold`.
You can avoid this collision with shell aliases, shell functions, and/or `PATH` changes.
Here is an example with the `o` dropped from `gfold`:

```shell
alias gfld=$HOME/.cargo/bin/gfold
```

### Upstream `libgit2` Issue

If you are seeing `unsupported extension name extensions.worktreeconfig` or similar errors, it may be related to
[libgit2/libgit2#6044](https://github.com/libgit2/libgit2/issues/6044).

This repository's tracking issue is [#205](https://github.com/nickgerace/gfold/issues/205).

## Community

For more information and thanks to contributors, users, and the "community" at large, please refer to the **[THANKS](./docs/THANKS.md)** file.

Name | Type | Description
--- | --- | ---
[Arch Linux community repository](https://archlinux.org/packages/community/x86_64/gfold/) | packaging | the `gfold` package _(note: before moving to the community repository, the [AUR](https://github.com/orhun/PKGBUILDs) was previously used for distribution)_
["One Hundred Rust Binaries"](https://www.wezm.net/v2/posts/2020/100-rust-binaries/page2/) | article | featured `gfold`
[nixpkgs](https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/version-management/gfold/default.nix) | packaging | the `gfold` package
[nvim-gfold.lua](https://github.com/AckslD/nvim-gfold.lua) | project | a `neovim` plugin for `gfold` _([announcement Reddit post](https://www.reddit.com/r/neovim/comments/t209wy/introducing_nvimgfoldlua/))_
