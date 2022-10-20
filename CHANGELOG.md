# Changelog

For new changes prior to version 4.0.0, please see [CHANGELOG_PRE_V4](./docs/CHANGELOG_PRE_V4.md).

- All notable changes to this project will be documented in this file
- All changes are from [@nickgerace](https://github.com/nickgerace) unless otherwise specified
- The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

## Unreleased

The latest version contains all changes.

## 4.1.0 - 2022-10-20

- Add debug symbol stripping for `cargo install` users (result: ~79% of the size of `4.0.1`)

### Changed

- Bump dependencies
- Change CLI library from `argh` to `clap v4`
- Ensure integration test artifacts exist in the correct location
- Refactor to use `cargo` workspaces, which should unlock the ability to create "scripts" via sub-crates

### Removed

- Remove ability to print the version as a single JSON field (combine `-V/--version` with `-d/--display json`)
  - Normally, this would necessitate a bump of the "major" field in the version, but `-V/--version` is serializable to JSON (just a string)

## 4.0.1 - 2022-07-05

### Changed

- Bump dependencies

## 4.0.0 - 2022-05-10

### Added

- Add [Bors](https://bors.tech/) to avoid merge skew/semantic merge conflicts
- Add color mode option with the following choices: "always", "compatibility" and "never" 
  - "always": display with rich colors (default)
  - "compatibility": display with portable colors
  - "never": display with no color
- Add display flag with the following choices: "standard" (or "default"), "json" and "classic"
  - "standard" (or "default") and "classic" output options return from the previous release
  - "json" output is a new option that displays all results in valid JSON, which is useful for third party applications, plugins, parsers, etc.
- Add documentation comments almost everywhere for `cargo doc`
- Add [git2-rs](https://github.com/rust-lang/git2-rs), which replaces `git` subcommand usage
  - Even though `git` subcommands were used over **git2-rs** to reduce binary size, significant speed increases could only be achieved by using the latter
  - More consistent behavior since git2-rs can be tested at a locked version
- Add JSON output flag for both version and results printing
- Add roubleshooting section to CLI help
- Add troubleshooting section to README for using `RUST_LOG` and `RUST_BACKTRACE`

### Changed

- Change config file location from `<prefix>/gfold/gfold.json` to `<prefix>/gfold.toml`
- Change config file type from JSON to TOML
- Change CLI help sections to be divided by headers
- Drastically improve performance by moving from sequential target generation to nested, parallel iterators for target generation
- Modify grey color default to avoid a bug where the `stdout` color is not refreshed within `tmux` when using macOS `Terminal.app`
- Refactor module layout
  - `display` now contains its child, `color`
  - `report` now contains its child, `target`
- Refactor testing for the entire crate
  - All tests have been replaced in favor on one integration test
  - The old tests relied on developer's environment, which is highly variable
  - The new test creates multiple files, directories, and repositories in the `target` directory to simulate an actual development environment
- Use a harness for the `color` module instead of individual functions

### Removed

- Remove debug flag in favor of using `RUST_LOG`
- Remove display of `none` fields for the standard (default) display of result (i.e. now, if an optional field was not found, it is not shown)
- Remove git path option for CLI and config file
- Remove `git` subcommand usage

### Notes

- Substantial performance gains should be noticeable in certain scenarios
  - Observed range in _loose_ benchmarking "real world" usage: ~1.2x to ~5.1x faster than `gfold 3.0.0` on macOS 12.3.1
  - Binary size has increased, but speed has taken priority for this release
- Using `RUST_LOG` and `RUST_BACKTRACE` should be more helpful when debugging unexpected output, performance or suspected bugs

## Before 4.0.0

Please see [CHANGELOG_PRE_V4](./docs/CHANGELOG_PRE_V4.md).
