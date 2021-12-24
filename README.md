# gfold

[![tag](https://img.shields.io/github/v/tag/nickgerace/gfold?sort=semver&logo=github&label=version&style=flat-square&color=blue)](https://github.com/nickgerace/gfold/releases/latest)
[![crates.io](https://img.shields.io/crates/v/gfold?style=flat-square&logo=rust&color=orange)](https://crates.io/crates/gfold)
[![build](https://img.shields.io/github/workflow/status/nickgerace/gfold/merge/main?style=flat-square)](https://github.com/nickgerace/gfold/actions?query=workflow%3Amerge+branch%3Amain)
[![license](https://img.shields.io/github/license/nickgerace/gfold?style=flat-square&color=purple)](./LICENSE)

> This **README** is for `gfold 3.x` users.
> Since `gfold 3.x` has not yet been released, contents of this **README** may be inapplicable to your version of `gfold`.
>
> For the latest, full release of `gfold 2.x`, please refer to the [**README** corresponding to the latest, full release](https://github.com/nickgerace/gfold/blob/2.0.2/README.md).

`gfold` is a CLI-driven application that helps you keep track of multiple Git repositories.

```
% gfold
📡 astrid ⇒ /home/neloth/src/astrid
unclean (main)
git@github.com:db/astrid.git
neloth@housetelvanni.dev

📡 fev ⇒ /home/neloth/src/fev
bare (issue2277)
none
neloth@housetelvanni.dev

📡 gb ⇒ /home/neloth/src/gb
unpushed (dev)
https://github.com/hrothgar/gb.git
neloth@housetelvanni.dev

📡 pam ⇒ /home/neloth/src/pam
clean (main)
https://github.com/onc/pam.git
neloth@solstheimcommunityserver.org
```

The classic display mode can be toggled on with `--classic`.

```
% gfold --classic
astrid  unclean   main       git@github.com:db/astrid.git
fev     bare      main       none
gb      unpushed  dev        https://github.com/hrothgar/gb.git
neloth  unclean   patch      git@github.com:telvanni/neloth.git
pam     clean     main       https://github.com/onc/pam.git
prime   clean     issue2287  git@github.com:bos/prime.git
```

If you'd prefer to use the classic display mode by default, and avoid setting the flag every time, you can set it in the config file (see **Usage** section).

## Description

This app displays relevant information for multiple Git repositories in one to many directories.
While this tool might seem limited in scope and purpose, that is by design.

By default, `gfold` looks at every Git repository via traversal from the current working directory.
However, if you would like to target another directory, you can pass that path (relative or absolute) as the first argument or change the default path in the config file.

## Installation

There are multiple methods for installing `gfold`.

### Homebrew (macOS only)

You can use [Homebrew](https://brew.sh) to install the [tap](https://github.com/nickgerace/homebrew-nickgerace/blob/main/Formula/gfold.rb).

```bash
brew install nickgerace/nickgerace/gfold
```

**Please note:** the tap may not work with [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux).

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
curl https://raw.githubusercontent.com/nickgerace/gfold/main/scripts/install.sh | bash
```

To uninstall `gfold` fully, after using this installation method, execute the following script:

```bash
curl https://raw.githubusercontent.com/nickgerace/gfold/main/scripts/uninstall.sh | bash
```

The uninstall script can also be used for cleanup in the event of a failed install.

#### Security Considerations

Please note that the installation convenience script _does not verify the binary with a checksum_.
Discretion is advised, including downloading and reading the script before execution.

## Usage

Pass in `--help` flag to see all the options for using this application.

```bash
gfold
gfold ..
gfold $HOME
gfold ~/
gfold /this/is/an/absolute/path
gfold ../../this/is/a/relative/path
```

### Config File

Upon execution, `gfold` will look for a config file at the following path on macOS, Linux and similar operating systems:

```bash
$HOME/.config/gfold/gfold.json
```

On Windows, the config file is located at the following path:

```powershell
{FOLDERID_Profile}\.config\gfold\gfold.json
```

Creating and using the config file is entirely optional, and you can ignore your config file at any time using the `-i` flag.

#### Example: Creating a Config File

Here is an example creation workflow for a config file.
This config file will default to the classic display mode and set the default path to `$HOME`, rather than the current working directory.

```bash
gfold --classic ~/ --print > $HOME/.config/gfold/gfold.json
```

Here are the contents of the resulting config file:

```json
{
  "path": "/home/neloth",
  "display_mode": "Classic"
}
```

#### Example: Backing Up a Config file

You can back up a config file and track its history with `git`.
On macOS, Linux, and most systems, you can link the file back to a `git` repository.

```bash
ln -s path/to/repository/gfold.json $HOME/.config/gfold/gfold.json
```

Now, you can update the config file within your repository and include the linking as part of your environment setup workflow.

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
