# gfold

[![GitHub](https://img.shields.io/github/license/nickgerace/gfold?style=flat-square)](./LICENSE)
[![Latest SemVer GitHub Tag](https://img.shields.io/github/v/tag/nickgerace/gfold?label=version&style=flat-square)](https://github.com/nickgerace/gfold/releases/latest)
[![Build Status](https://img.shields.io/github/workflow/status/nickgerace/gfold/merge/main?style=flat-square)](https://github.com/nickgerace/gfold/actions?query=workflow%3Amerge+branch%3Amain)

```gfold``` is a CLI application that helps you keep track of multiple Git repositories.

```bash
user at hostname in ~/git
% gfold
great-journey      unclean  main      git@github.com:truth/great-journey.git
installation-zero  bare     main      https://github.com/the-ark/installation-zero.git
sierra             clean    dev       https://github.com/forward-unto-dawn/sierra.git
spark              clean    issue343  git@github.com:guilty/spark.git
tartarus           unclean  delta     git@github.com:covenant/tartarus.git
voi                clean    main      https://github.com/earth/voi.git
```

## Description and Motivation

This app displays relevant information for multiple Git repositories in one, or multiple, directories.
While this tool might seem limited in scope and purpose, that is by design.

It prints each repository in alphabetical order, and pads each result based on the longest directory, branch, and status string.
By default, ```gfold``` looks at every Git repository in the current working directory.
However, if you would like to target another directory, you can pass that path (relative or absolute) as the first argument.

## Installation

There are multiple ways to install ```gfold```, but here are the currently recommended methods...

### 1) Download a GitHub Binary

Most likely, the easiest method of obtaining ```gfold``` is via the [latest GitHub release](https://github.com/nickgerace/gfold/releases/latest).

Once you have it downloaded, you can add it to your ```PATH```.
Here is an example on how to do that on macOS and Linux...

```bash
chmod +x ./gfold
sudo mv ./gfold /usr/local/bin/
```

*Note*: you may have to reload your shell in order to see ```gfold``` in your ```PATH```.

#### Advanced Management

You can use symbolic links to swap between versions, and manage multiple at a time.
Here is a full install workflow...

```bash
wget https://github.com/nickgerace/gfold/releases/download/$VERSION/gfold-linux-gnu-amd64)
mv gfold-linux-gnu-amd64 gfold-$VERSION
chmod +x ./gfold-$VERSION
sudo mkdir /usr/local/gfold/
sudo mv ./gfold-$VERSION /usr/local/gfold/
ln -s /usr/local/gfold/gfold-$VERSION /usr/local/bin/gfold
```

Now, you can add/remove versions of the binary from ```/usr/local/gfold/```, and change the symbolic link as needed.

### 2) AUR (Arch User Repository)

This application is available for all Linux distributions that support installing packages from the AUR.
Special thanks to [orhun](https://github.com/orhun) for [maintaining](https://github.com/orhun/PKGBUILDs) these packages.

- [gfold](https://aur.archlinux.org/packages/gfold/) (builds from source)
- [gfold-git](https://aur.archlinux.org/packages/gfold-git/) (VCS/development package)

**Note**: many people choose to use an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers), such as [yay](https://github.com/Jguer/yay) (example: ```yay -S gfold```), in order to install their AUR packages.

### 3) Cargo Install

You can build from source with ```cargo``` by executing the following...

```bash
cargo install --git https://github.com/nickgerace/gfold --tag 0.5.1
```

## Usage

For all the ways on how to use this application, pass in the ```--help``` flag.

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

All external crates were vetted for support on all three major desktop platforms.
```gfold``` is tested for the latest versions of the following systems, but may work on more...

- **Linux**: ```linux-gnu-amd64```
- **macOS**: ```darwin-amd64```
- **Windows 10**: ```windows-amd64```

## Changelog

Please check out [CHANGELOG.md](./CHANGELOG.md) for more information.
It follows the [Keep a Changelog](https://keepachangelog.com/) format.

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- License: [Apache 2.0](./LICENSE)

## Special Thanks

- [@yaahc](https://github.com/yaahc) (mentoring)
- [@orhun](https://github.com/orhun) (maintaining AUR packages)
- [@jrcichra](https://github.com/jrcichra) (adding multi-OS support to the original CI pipeline)
