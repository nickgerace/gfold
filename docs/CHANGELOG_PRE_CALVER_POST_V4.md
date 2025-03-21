# Changelog From Version 4.0.0 to CalVer 

For new changes, please see the current [CHANGELOG](../CHANGELOG.md).

- All notable, released changes to this project from a user's perspective will be documented in this file
- All changes are from [@nickgerace](https://github.com/nickgerace) unless otherwise specified
- The format was inspired by [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
- This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

## After 4.6.0

Please see [CHANGELOG](../CHANGELOG.md).

## 4.6.0 - 2024-12-10

### Added

- Add XDG-first user directory lookup using [`user_dirs`](https://github.com/uncenter/user_dirs/blob/193547d1d2f190dbc6fbf9f29a4aa2d4318070db/README.md) from [@uncenter](https://github.com/uncenter)

### Changed

- Bump dependencies
- Help message to rely on line wrapping
- Update release binary names for clarity (including fixing the macOS one for the correct architecture)

## 4.5.1 - 2024-12-09

### Changed

- Bump dependencies

## 4.5.0 - 2024-05-23

### Added

- Ability to use the standard display mode, but the results are purely alphabetical and not sorted by repository status

### Changed

- Bump dependencies

## 4.4.1 - 2023-12-23

### Changed

- Bump dependencies

## 4.4.0 - 2023-06-26

### Changed

- Bump dependencies

### Notes

- Bump the minor version field instead of the patch field because the dependency tree has significantly changed
  - Technically, the sole user-facing change is that the external dependencies have been bumped
  - For context, `libgfold` was newly introduced and contains the majority of the original `gfold` source code
- Only run CI checks on merge
- Publish and use `libgfold` for the first time
- Split `gfold` into two crates: a library and a binary
- Use cargo workspace dependencies

## 4.3.3 - 2023-04-07

### Changed

- Bump dependencies

### Notes

- Remove `flake.nix` due to lack of use (might return in the future)
- Fix missing `4.3.2` title in the `CHANGELOG`

## 4.3.2 - 2023-03-09

### Changed

- Bump dependencies

### Notes

- Add `flake.nix` for more local development options

## 4.3.1 - 2023-02-05

### Changed

- Bump dependencies
- Bump LICENSE year

## 4.3.0 - 2023-02-05

### Added

- Add submodule information to the `json` display mode (i.e. `gfold -d json`)
  - This information is not yet accessible in other display modes

### Changed

- Bump dependencies

### Notes

- Add demo GIF to README
- Performed significant refactor to reduce the usage of "floating" functions
  (i.e. ensure functions are members of unit structs at minimum) as well as
  remove reliance on a single generic error enum

## 4.2.0 - 2022-12-21

### Changed

- Add "unknown" status for repositories hitting the `extensions.worktreeconfig` error
- Bump dependencies
- Change "unpushed" color to blue
- Ignore the `extensions.worktreeconfig` error until the corresponding upstream issue is resolved: https://github.com/libgit2/libgit2/issues/6044

## 4.1.2 - 2022-12-20

### Changed

- Bump dependencies
- When checking if "unpushed" and attempting to resolve the reference from a short name, ignore the error and assume we need to push

## 4.1.1 - 2022-12-19

### Changed

- Bump dependencies
- Ensure dependencies have their minor version fields locked

### Notes

- This `CHANGELOG` entry was accidentally not included in the `4.1.1` tag

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

Please see [CHANGELOG_PRE_V4](./CHANGELOG_PRE_V4.md).
