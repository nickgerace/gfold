# gfold

[![latest release tag](https://img.shields.io/github/v/tag/nickgerace/gfold?sort=semver&logo=git&logoColor=white&label=version&style=for-the-badge&color=blue)](https://github.com/nickgerace/gfold/releases/latest)
[![crates.io version](https://img.shields.io/crates/v/gfold?style=for-the-badge&logo=rust&color=orange)](https://crates.io/crates/gfold)
[![build status](https://img.shields.io/github/actions/workflow/status/nickgerace/gfold/ci.yml?branch=main&style=for-the-badge&logo=github&logoColor=white)](https://github.com/nickgerace/gfold/actions)
[![calver](https://img.shields.io/badge/calver-YYYY.MM.MICRO-cyan.svg?style=for-the-badge)](https://calver.org)
[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

`gfold` is a CLI tool that helps you keep track of multiple Git repositories.

[![A GIF showcasing gfold in action](https://raw.githubusercontent.com/nickgerace/gfold/main/assets/demo.gif)](https://raw.githubusercontent.com/nickgerace/gfold/main/assets/demo.gif)

If you'd prefer to use the classic display mode by default, and avoid setting the flag every time, you can set it in the config file (see **Usage** section).

## Announcement (February 2025)

All releases now follow the [CalVer](https://calver.org/) versioning scheme, starting with `2025.2.1`.
This change is both forwards and backwards compatible with the [Semantic Versioning](https://semver.org/spec/v2.0.0.html) versioning scheme, which was used from the first release through version `4.6.0`.

*No end user action is required specifically for the versioning scheme change itself.*

This announcement will be eventually removed from this [README](./README.md) and will eventually be moved into the [CHANGELOG](./CHANGELOG.md).

## Description

This app displays relevant information for multiple Git repositories in one to many directories.
It only reads from the filesystem and will never write to it.
While this tool might seem limited in scope and purpose, that is by design.

By default, `gfold` looks at every Git repository via traversal from the current working directory.
If you would like to target another directory, you can pass its path (relative or absolute) as the first argument or change the default path in the config file.

After traversal, `gfold` leverages [rayon](https://github.com/rayon-rs/rayon) to perform concurrent, read-only analysis of all Git repositories detected.
Analysis is performed by leveraging the [git2-rs](https://github.com/rust-lang/git2-rs) library.

## Usage

Provide the `-h/--help` flag to see all the options for using this application.

```shell
# Operate in the current working directory or in the location provided by a config file, if one exists.
gfold

# Operate in the parent directory.
gfold ..

# Operate in the home directory (first method).
gfold $HOME

# Operate in the home directory (second method).
gfold ~/

# Operate with an absolute path.
gfold /this/is/an/absolute/path

# Operate with a relative path.
gfold ../../this/is/a/relative/path

# Operate with three paths.
gfold ~/src ~/projects ~/code
```

### Config File

If you find yourself providing the same arguments frequently, you can create and use a config file.
`gfold` does not come with a config file by default and config files are entirely optional.

How does it work?
Upon execution, `gfold` will look for a config file at the following paths (in order):

- `$XDG_CONFIG_HOME/gfold.toml`
- `$XDG_CONFIG_HOME/gfold/config.toml`
- `$HOME/.config/gfold.toml`

`$XDG_CONFIG_HOME` refers to the literal `XDG_CONFIG_HOME` environment variable, but will default to the appropriate operating system-specific path if not set (see [`user_dirs`](https://github.com/uncenter/user_dirs) for more information).

If a config file is found, `gfold` will read it and use the options specified within.

For config file creation, you can use the `--dry-run` flag to print valid TOML.
Here is an example config file creation workflow on macOS, Linux and similar platforms:

```shell
gfold -d classic -c never ~/ --dry-run > $HOME/.config/gfold.toml
```

Here are the contents of the resulting config file:

```toml
paths = ['/home/neloth']
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
ln -s <path-to-repository>/gfold.toml $HOME/.config/gfold.toml
```

Now, you can update the config file within your repository and include the linking as part of your environment setup workflow.

## Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/gfold.svg)](https://repology.org/project/gfold/versions)

### Homebrew (macOS and Linux)

You can use [Homebrew](https://brew.sh) to install `gfold` using the [core formulae](https://formulae.brew.sh/formula/gfold).

However, you may run into a naming collision on macOS if [coreutils](https://formulae.brew.sh/formula/coreutils) is installed via `brew`.
See the [troubleshooting](#troubleshooting-and-known-issues) section for a workaround and more information.

```shell
brew install gfold
```

### Arch Linux

You can use [pacman](https://wiki.archlinux.org/title/Pacman) to install `gfold` from the [extra repository](https://archlinux.org/packages/extra/x86_64/gfold/).

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

### Cargo

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

### Deprecated: Homebrew Tap (macOS only)

The [tap located at nickgerace/homebrew-nickgerace](https://github.com/nickgerace/homebrew-nickgerace/blob/main/Formula/gfold.rb) has been deprecated.
Please use the aforementioned core Homebrew package instead.

### Preferred Installation Method Not Listed?

Please [file an issue](https://github.com/nickgerace/gfold/issues/new)!

## Compatibility

`gfold` is intended to be ran on _any_ tier one Rust 🦀 target.
Please [file an issue](https://github.com/nickgerace/gfold/issues) if your platform is unsupported.

## Troubleshooting and Known Issues

If you encounter unexpected behavior or a bug and would like to see more details, please run with increased verbosity.

```shell
gfold -vvv
```

If the issue persists, please [file an issue](https://github.com/nickgerace/gfold/issues).
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

For more information and thanks to users and the "community" at large, please refer to the **[COMMUNITY THANKS](./docs/COMMUNITY_THANKS.md)** file.

- [Packages for NixOS, Arch Linux and more](https://repology.org/project/gfold/versions)
- ["One Hundred Rust Binaries"](https://www.wezm.net/v2/posts/2020/100-rust-binaries/page2/), an article that featured `gfold`
- [nvim-gfold.lua](https://github.com/AckslD/nvim-gfold.lua), a `neovim` plugin for `gfold` _([announcement Reddit post](https://www.reddit.com/r/neovim/comments/t209wy/introducing_nvimgfoldlua/))_
