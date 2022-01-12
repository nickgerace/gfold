# gfold

[![build](https://img.shields.io/github/workflow/status/nickgerace/gfold/merge/main?style=flat-square&logo=github&logoColor=white)](https://github.com/nickgerace/gfold/actions?query=workflow%3Amerge+branch%3Amain)
[![tag](https://img.shields.io/github/v/tag/nickgerace/gfold?sort=semver&logo=git&logoColor=white&label=version&style=flat-square&color=silver)](https://github.com/nickgerace/gfold/releases/latest)
[![crates.io](https://img.shields.io/crates/v/gfold?style=flat-square&logo=rust&color=orange)](https://crates.io/crates/gfold)
[![arch linux](https://img.shields.io/archlinux/v/community/x86_64/gfold?logo=archlinux&logoColor=white&style=flat-square&color=blue)](https://archlinux.org/packages/community/x86_64/gfold/)
[![license](https://img.shields.io/github/license/nickgerace/gfold?style=flat-square&logo=apache&color=silver)](./LICENSE)

`gfold` is a CLI-driven application that helps you keep track of multiple Git repositories.

```
% gfold
astrid ~ /home/neloth/src/astrid
  unclean (main)
  git@github.com:db/astrid.git
  neloth@housetelvanni.dev
fev ~ /home/neloth/src/fev
  bare (issue2277)
  none
  neloth@housetelvanni.dev
gb ~ /home/neloth/src/gb
  unpushed (dev)
  https://github.com/hrothgar/gb.git
  neloth@housetelvanni.dev
pam ~ /home/neloth/src/pam
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
If you would like to target another directory, you can pass its path (relative or absolute) as the first argument or change the default path in the config file.

After traversal, `gfold` leverages [rayon](https://github.com/rayon-rs/rayon) to perform concurrent, read-only analysis of all Git repositories detected.
Analysis is performed with the `git` CLI rather than [libgit2](https://github.com/rust-lang/git2-rs) to reduce binary size.

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

Here is an example creation workflow for a config file:

```bash
gfold --classic ~/ --print > $HOME/.config/gfold/gfold.json
```

This config file will default to the classic display mode and set the default path to `$HOME`, rather than the current working directory.

Here are the contents of the resulting config file:

```json
{
  "path": "/home/neloth",
  "display_mode": "Classic",
  "git_path": null
}
```

You can back up a config file and track its history with `git`.
On macOS, Linux, and most systems, you can link the file back to a `git` repository.

```bash
ln -s path/to/repository/gfold.json $HOME/.config/gfold/gfold.json
```

Now, you can update the config file within your repository and include the linking as part of your environment setup workflow.

## Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/gfold.svg)](https://repology.org/project/gfold/versions)

**macOS users:** you can use [Homebrew](https://brew.sh) to install the [tap](https://github.com/nickgerace/homebrew-nickgerace/blob/main/Formula/gfold.rb).

```bash
brew install nickgerace/nickgerace/gfold
```

_Note:_ the tap may not work with [Linuxbrew](https://docs.brew.sh/Homebrew-on-Linux).

**Arch Linux users:** you can use [pacman](https://wiki.archlinux.org/title/Pacman) to install `gfold` from the [community repository](https://archlinux.org/packages/community/x86_64/gfold/).

```bash
pacman -S gfold
```

If you'd like the [development (VCS) package](https://aur.archlinux.org/packages/gfold-git/), you can install it from the AUR.

```bash
paru -S gfold-git
```

_Note:_ the above example uses [paru](https://github.com/Morganamilo/paru), which is an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers) used to install packages from the AUR.

**Nix and NixOS users:** you can install `gfold` from [nixpkgs](https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/version-management/git-and-tools/gfold/default.nix):

```bash
nix-env --install ripgrep
```

**Rust developers and Cargo users:** you can use [cargo](https://crates.io) to install the [crate](https://crates.io/crates/gfold) on almost any platform.

```bash
cargo install gfold
```

Keeping the crate up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).

```bash
cargo install cargo-update
cargo install-update -a
```

**Build and install from source:** if you want to install from source, and not from [crates.io](https://crates.io/crates/gfold), you can clone the repository and build `gfold`.

```bash
(
    git clone https://github.com/nickgerace/gfold.git
    cd gfold
    make install
)
```

**Download a binary:** if you do not want to use one of the above installation methods, you can download a binary from the [releases](https://github.com/nickgerace/gfold/releases) page.

```bash
curl -s https://raw.githubusercontent.com/nickgerace/gfold/main/scripts/install.sh | bash
```

_Note:_ the installation convenience script _does not verify the binary with a checksum_.
Discretion is advised, including downloading and reading the script before execution.

To uninstall `gfold` fully, after using this installation method, execute the following script:

```bash
curl -s https://raw.githubusercontent.com/nickgerace/gfold/main/scripts/uninstall.sh | bash
```

The uninstall script can also be used for cleanup in the event of a failed install.

**Preferred package manager not listed:** please [file an issue](https://github.com/nickgerace/gfold/issues/new/choose)!

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
