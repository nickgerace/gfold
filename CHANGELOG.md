# Changelog

All notable changes to this project will be documented in this file.
All changes are from [@nickgerace](https://github.com/nickgerace) unless otherwise specified.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

<!-- The latest version contains all changes. -->

### Added

- [git2-rs](https://github.com/rust-lang/git2-rs), which replaces `git` subcommand usage

### Changed

- Config file location from `<prefix>/gfold/gfold.json` to `<prefix>/gfold.toml`
- Config file type from JSON to TOML
- Major performance improvements due to moving from sequential target generation to nested, parallel iterators for target generation

### Removed

- Git path option for CLI and config file
- `git` subcommand usage

### Notes

- Even though `git` subcommands were used over **git2-rs** to reduce binary size, significant speed increases could only be achieved by using the latter.
- Technically, removing the Git path option from the CLI and the config file could require a major version increase.
- Given the immaturity of `3.0.0`, the (likely) infrequent use of the Git path option, and the overall structure/behavior remaining intact, the removal of this config option only necessitates a minor version increase.
- For technical details on the field removal, please refer to the [diff between releases](https://github.com/nickgerace/gfold/compare/3.0.0...3.1.0).

### [3.0.0] - 2022-01-06

### Added

- Ability to ignore config file options
- Ability to print merged config options
- Ability to specify path to `git`
- Ability to store default path target in config file (defaults to current working directory)
- Ability to use config file in `$HOME/.config/gfold/gfold.json` and `{FOLDERID_Profile}\.config\gfold\gfold.json`
- Ability to use old display mode with `--classic` flag and store preference in config file
- Formal CLI parsing library, `argh`
- Install and uninstall scripts
- New display mode that avoids grouping repositories (API-breaking since this is the new default display mode)

### Changed

- Codebase to a domain-driven architecture (major refactor)

### Removed

- Mention of the [deprecated, old Homebrew tap](https://github.com/nickgerace/homebrew-gfold) in the README
- Short `-h` flag due to CLI crate addition (`argh`)

### Notes

- Evaluated using `tracing` and `tracing-subscriber` over `log` and `env_logger`, but due to their combined larger size, the logging crates remain the same as before.
- The config file can be non-existent, empty, partially filled out or completely filled out. There's also an option to ignore the config file completely and only use CLI options.
- This crate has used other CLI parsing libraries in the past, and recently did not use any, but with manual testing and [publicly available benchmarks](https://github.com/rust-cli/argparse-benchmarks-rs/blob/c37e78aabdaa4384a9c49be3735a686803d0e37a/README.md#results), `argh` is now in use.

## [2.0.2] - 2021-12-02

### Changed

- Misc package bumps

## [2.0.1] - 2021-10-29

### Added

- Logger that can be set via an environment variable (`RUST_LOG`)

### Changed

- Permission denied errors to be logged rather than displayed to `stderr`

### Misc

- Ensure `crates.io` and `git` tag are in sync (very slight and accidental derivation for `2.0.0`)

## [2.0.0] - 2021-10-29

### Added

- Discrete Code of Conduct file
- Unpushed commit checking by default (greedy static analysis, so this may need to be tuned for edge cases)
- `git` CLI wrapper instead of Git library usage due to security, Git already being installed, inconsistencies between the library and the CLI, and more

### Changed

- Codebase re-write geared towards data-efficiency and parallelism
- Dramatic runtime speed and binary size improvements (consistently able to reproduce, but heavily variable based on payload, OS, etc.)
- Entire structure from library-driven to application-driven (no `lib.rs`)

### Removed

- All CLI flags except for `-h/--help` and `-V/--version`
- CLI crate since it is unneeded
- Git library usage in favor of leveraging `git` subcommands due to security, Git already being installed, inconsistencies between the library and the CLI, and more
- `DEVELOPING.md` and `EXTRA.md` since they were outdated/unimportant
- `lib.rs` and the crate's library-based components

## [1.4.1] - 2021-08-02

### Changed

- Misc package bumps

## [1.4.0] - 2021-06-17

### Changed

- Continue upon PermissionDenied errors rather than exiting
- Documentation to be moved to the new `docs` directory

## [1.3.0] - 2021-05-25

### Changed

- Config type to be embedded within the Driver
  - Not in public library modules, but this should improve generation efficiency

### Removed

- `-d/--debug` flag since all logging has been removed from the librariy, and `main.rs` does not log
- `env_logger` crate
- `log` crate
- Logging from the entire library in favor of returning errors when needed and handling when possible/preferred

## [1.2.1] - 2021-05-23

### Changed

- Bold table headers instead of repo names

### Removed

- Extra newline before first entry when printing all tables

## [1.2.0] - 2021-05-16

### Removed

- Middleware `run` function in `lib.rs` since it's unecessary and unintuitive
  - Before, you used a type from the `driver` module as a parameter for `run`
  - Now, you only use types from `driver`

## [1.1.1] - 2021-05-16

### Changed

- Config management to be done by applications consuming the library (moved out of `lib.rs`)
- Driver module to be public
- Printing to STDOUT to be done by applications consuming the library (moved out of `lib.rs`)
- `TableWrapper` type to be a private, internal type

## [1.1.0] - 2021-05-15

### Added

- Shallow search flag to disable recursive search (accessible via `-s/--shallow`)

### Changed

- Binary size to be ~60% of the size of `gfold 1.0.4`
  - Primarily, this was achieved by removing unused default features from imported crates
  - Runtime speed is the same, better, or more consistent
- Default search behavior to be recursive rather than shallow
- Short flag for `--skip-sort` from `-s` to `-x`
- Workspace implementation to a single crate (similar to before `gfld`)

### Removed

- `gfld`, the lightweight version of `gfold` due the following reasons:
  - its over ~105% average slower runtime speed (despite it being ~40% of the size)
  - printing to STDOUT was not consistent in subcommand scenarios
- Recursive flag since `gfold` uses recursive search by default

## [1.0.4] - 2021-04-04

### Changed

- Fixed final output order (sorted by name, then by status)

## [1.0.3] - 2021-04-02

### Changed

- Directory name finder to default to the current working directory if empty (`gfld`)
- Misc. optimizations in `gfld`
- Release profile optimizations to be at workspace scope for binaries

## [1.0.2] - 2021-04-01

### Changed

- `gfld` output to not include parent root since all results started with the same string

## [1.0.1] - 2021-03-30

### Added

- `Cargo.lock` to the workspace to fix AUR builds

## Changed

- CI to use `--locked` for builds

## [1.0.0] - 2021-03-29

### Added

- A brand new, minimal CLI: `gfld`
- `DEVELOPING.md` for instructions on building `gfld`

### Changed

- Documentation to include `gfld`
- GitHub issue template to include `gfld` information
- GitHub PR CI to only build for Linux while keeping macOS and Windows for release CI
- The repository to be split into two crates: `gfold` and `gfld`
- Unnecessary `PathBuf` usages to `Path` when possible in `util.rs`

### Removed

- Release workflow for GitHub actions (now, it is "merge only")
- Uploaded binaries due to lack of checksums and maintenance

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
