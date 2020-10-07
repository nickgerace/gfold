# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

The latest version contains all changes.

## [0.5.1] - 2020-10-07

### Added

- Release dummy GitHub Action from [@nickgerace](https://github.com/nickgerace).
- Version README badge from [@nickgerace](https://github.com/nickgerace).

### Changed

- A reduction to CI build time and complexity by combining the test and check steps, from [@nickgerace](https://github.com/nickgerace).
- GitHub workflow "merge" file name to "merge.yml" from [@nickgerace](https://github.com/nickgerace).
- GitHub workflow name to "merge" from [@nickgerace](https://github.com/nickgerace).
- OS compatibility claims in README through a simplified list from [@nickgerace](https://github.com/nickgerace).
- README badges to use shields.io from [@nickgerace](https://github.com/nickgerace).

### Removed 

- MUSL mentions in docs from [@nickgerace](https://github.com/nickgerace).

## [0.5.0] - 2020-09-02

### Added

- Recursive search feature and flag from [@nickgerace](https://github.com/nickgerace).
- Skip sort feature and flag from [@nickgerace](https://github.com/nickgerace).
- Unit tests for recursive search and skip sort from [@nickgerace](https://github.com/nickgerace).
- AUR PKGBUILD GitHub repository to README from [@nickgerace](https://github.com/nickgerace).
- Results and TableWrapper structs, and relevant functions, from [@nickgerace](https://github.com/nickgerace).
- Three methods for Results struct (printing/sorting/populating results) from [@nickgerace](https://github.com/nickgerace).
- Make targets for ```run-recursive``` and ```install-local``` from [@nickgerace](https://github.com/nickgerace).

### Changed

- Switch from ```walk_dir``` function to object-driven harness for execution from [@nickgerace](https://github.com/nickgerace).
- Move ```walk_dir``` function logic to ```Results``` method from [@nickgerace](https://github.com/nickgerace).
- Function ```is_git_repo``` to its own file from [@nickgerace](https://github.com/nickgerace).
- Any unnecessary match block to use "expect" instead from [@nickgerace](https://github.com/nickgerace).
- Cargo install to use a specific tag from [@nickgerace](https://github.com/nickgerace).
- Version upgrade workflow to Makefile from [@nickgerace](https://github.com/nickgerace).

### Removed

- Leftover "FIXME" comments for recursive search ideas from [@nickgerace](https://github.com/nickgerace).

## [0.4.0] - 2020-08-31

### Added

- Changelog from [@nickgerace](https://github.com/nickgerace).
- Tags automation from [@nickgerace](https://github.com/nickgerace).

### Changed

- Example output to use mythical repositories from [@nickgerace](https://github.com/nickgerace).
- Path flag to positional argument from [@nickgerace](https://github.com/nickgerace).
- Switched to structopt library for CLI parsing from [@nickgerace](https://github.com/nickgerace).

### Removed

- Tag v0.3.0 (duplicate of 0.3.0 with the "v" character) from [@nickgerace](https://github.com/nickgerace).
- All GitHub releases before 0.3.1 from [@nickgerace](https://github.com/nickgerace).
- Releases information from README from [@nickgerace](https://github.com/nickgerace).

## [0.3.1] - 2020-08-30

### Added

- Add AUR installation documentation from [@nickgerace](https://github.com/nickgerace).
- Add AUR packages from [@orhun](https://github.com/orhun).

### Changed

- Switch to Apache 2.0 license from MIT license from [@nickgerace](https://github.com/nickgerace).
- Reorganize build tags, and add test build target from [@nickgerace](https://github.com/nickgerace).

## [0.3.0] - 2020-08-24

### Changed

- Handling for bare repositories to print their status to STDOUT from [@nickgerace](https://github.com/nickgerace) with the mentorship of [@yaahc](https://github.com/yaahc).

## [0.2.2] - 2020-08-24

### Changed

- "Continue" to the next repository object if the current object is bare from [@nickgerace](https://github.com/nickgerace).
- Release availability in README from [@nickgerace](https://github.com/nickgerace).

## [0.2.1] - 2020-06-08

### Added

- Experimental statically-linked, MUSL support from [@nickgerace](https://github.com/nickgerace).

## [0.2.0] - 2020-05-10

### Changed

- Switched to prettytable-rs from [@nickgerace](https://github.com/nickgerace).
- Unit tests to work with prettytable-rs from [@nickgerace](https://github.com/nickgerace).

## [0.1.1] - 2020-04-10

### Added

- Example output, contributors, and usage in README from [@nickgerace](https://github.com/nickgerace).
- Building for Windows, macOS, and Linux amd64 in CI pipeline from [@jrcichra](https://github.com/jrcichra).

## [0.1.0] - 2020-04-08

### Added

- Base contents from [@nickgerace](https://github.com/nickgerace).
