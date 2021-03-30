# Changelog

All notable changes to this project will be documented in this file.
All changes are from [@nickgerace](https://github.com/nickgerace) unless otherwise specified.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

<!-- The latest version contains all changes. -->

### Added

- A brand new, minimal CLI: `gfld`
- `DEVELOPING.md` for instructions on building `gfld`

### Changed

- Documentation to include `gfld`
- GitHub issue template to include `gfld` information
- GitHub PR CI to only build for Linux while keeping macOS and Windows for release CI
- The repository to be split into two crates: `gfold` and `gfld`
- Unnecessary `PathBuf` usages to `Path` when possible in `util.rs`

## [0.9.1] - 2021-03-16

### Added

- RELEASE file for releasing `gfold`

### Changed

- README installation section to be condensed
- LICENSE to not use copyright dates or name (reduce maintenance)

### Removed

- Makefile in order to be cross-platform-friendly

## [0.9.0] - 2021-02-15

### Added

- Email display feature
- Include standard directory feature
- Shorthand flag for all features without one

### Changed

- Directory walking to skip hidden directories
- Repository opening check to log error in debug mode rather than panic

### Removed

- File header comments
- Prettytable macros

## [0.8.4] - 2021-01-26

### Added

- Dependencies section to CHANGELOG
- `paru` to suggested AUR helpers in README

### Changed

- All CRLF files to LF
- Condense tests into loops where possible
- Label `unpush_check` as an experimental feature
- `macos-amd64` to `darwin-amd64`
- `unpush_check` from `disable` to `enable`

## [0.8.3] - 2020-12-15

### Added

- Disable unpushed commit check flag and functionality
- Logging for origin and local reference names for unpushed commit check

## [0.8.2] - 2020-12-14

### Added

- `gfold --version` to issue template
- Unpush functionality (again)

### Changed

- Unpush function to only return boolean

### Removed

- Contributing section from README to reduce requirements
- Empty results message since it was potentially misleading

## [0.8.1] - 2020-12-01

### Added

- Condition enum for adding rows to final table
- Debug flag
- Many debug statements for the new debug flag

### Changed

- Bare repository checking to original behavior
- `util.rs` results generation to include Condition enum

### Removed

- Carets from `Cargo.toml` to maintain stability
- Unpush functionality temporarily

## [0.8.0] - 2020-11-26

### Added

- Debugging calls for general usage and the new unpushed commit code
- Derive debug to the `Config` struct
- Lightweight logging stack with `env_logger` and `log`
- Two files: `driver.rs` and `util.rs`
- Unpushed commit status functionality and output

### Changed

- Bare repository detection to use upstream function
- Library contents into `driver.rs` and `util.rs` through a major refactor

## [0.7.1] - 2020-11-18

### Added

- In-depth description of the `run` function

### Changed

- Consolidated boolean test permutations into one test

### Removed

- All non-public comments in `*.rs` files

## [0.7.0] - 2020-11-11

### Added

- Crate to crates.io
- Crates.io publishing requirements to `[package]` in `Cargo.toml`
- Homebrew tap
- Library description to `lib.rs`

### Changed

- Dependency versioning to use carets
- README mentions of specific version tags
- README plaintext blocks to single quotes when mixed with formatted text
- README to sort installation method by package managers first

### Removed

- Public structs and functions without only `run` (primary backend driver) remaining

## [0.6.2] - 2020-11-03

### Added

- No color flag and functionality

### Removed

- Pull request template

## [0.6.1] - 2020-10-12

### Added

- Code of Conduct link
- GitHub issue template
- GitHub pull request template

### Changed

- LICENSE to be extended through 2021
- Match blocks in `lib.rs` to be consolidated
- Nearly all contents of `lib.rs` to return errors back to the calling function in `main.rs`

### Removed

- Duplicate code related to the match block consolidation

## [0.6.0] - 2020-10-10

### Added

- Doc comments and `cargo doc` to `release` target
- `eyre` for simple backtrace reporting
- `gfold-bin` to AUR portion of README
- `lib.rs` as part of major refactor

### Changed

- Pre-build Makefile targets to be consolidated
- Refactor source code to be driven by a library, helmed by `lib.rs`

### Removed

- `util.rs` and `gfold.rs` as part of major refactor

## [0.5.2] - 2020-10-08

### Added

- GitHub release downloads to README
- Binary publishing workflow to the dummy file

### Changed

- Existing merge workflow to use debug building instead of release building
- Makefile target containing the old default branch name

### Removed

- Makefile target for statically-linked building

## [0.5.1] - 2020-10-07

### Added

- Release dummy GitHub Action
- Version README badge

### Changed

- A reduction to CI build time and complexity by combining the test and check steps,
- GitHub workflow "merge" file name to "merge.yml"
- GitHub workflow name to "merge"
- OS compatibility claims in README through a simplified list
- README badges to use shields.io

### Removed 

- MUSL mentions in docs

## [0.5.0] - 2020-09-02

### Added

- Recursive search feature and flag
- Skip sort feature and flag
- Unit tests for recursive search and skip sort
- AUR PKGBUILD GitHub repository to README
- Results and TableWrapper structs, and relevant functions,
- Three methods for Results struct (printing/sorting/populating results)
- Make targets for `run-recursive` and `install-local`

### Changed

- Switch from `walk_dir` function to object-driven harness for execution
- Move `walk_dir` function logic to `Results` method
- Function `is_git_repo` to its own file
- Any unnecessary match block to use "expect" instead
- Cargo install to use a specific tag
- Version upgrade workflow to Makefile

### Removed

- Leftover "FIXME" comments for recursive search ideas

## [0.4.0] - 2020-08-31

### Added

- Changelog
- Tags automation

### Changed

- Example output to use mythical repositories
- Path flag to positional argument
- Switched to structopt library for CLI parsing

### Removed

- Tag v0.3.0 (duplicate of 0.3.0 with the "v" character)
- All GitHub releases before 0.3.1
- Releases information from README

## [0.3.1] - 2020-08-30

### Added

- Add AUR installation documentation
- Add AUR packages from [@orhun](https://github.com/orhun)

### Changed

- Switch to Apache 2.0 license from MIT license
- Reorganize build tags, and add test build target

## [0.3.0] - 2020-08-24

### Changed

- Handling for bare repositories to print their status to STDOUT with the mentorship of [@yaahc](https://github.com/yaahc)

## [0.2.2] - 2020-08-24

### Changed

- "Continue" to the next repository object if the current object is bare
- Release availability in README

## [0.2.1] - 2020-06-08

### Added

- Experimental statically-linked, MUSL support

## [0.2.0] - 2020-05-10

### Changed

- Switched to prettytable-rs
- Unit tests to work with prettytable-rs

## [0.1.1] - 2020-04-10

### Added

- Example output, contributors, and usage in README
- Building for Windows, macOS, and Linux amd64 in CI pipeline from [@jrcichra](https://github.com/jrcichra)

## [0.1.0] - 2020-04-08

### Added

- Base contents
