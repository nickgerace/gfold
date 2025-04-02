# Changelog

- All notable, released changes to this project from a user's perspective will be documented in this file
- All changes are from [@nickgerace](https://github.com/nickgerace) unless otherwise specified
- The format was inspired by [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## Versioning Scheme

This project follows [CalVer](https://calver.org/) for its versioning scheme, starting with `2025.2.1`.
It used to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html) from the first release through version `4.6.0`.
This versioning approach is both backwards and forwards compatible with Semantic Versioning.

Here is the template for the scheme:

```
<YYYY>.<MM>.<RELEASE-NUMBER>
```

- The first field, `YYYY`, refers to the year of release, specified via four digits.
- The second field, `MM`, refers to the month of release, specified via one (January through September) or two digits (October through December).
- The third field, `RELEASE-NUMBER`, refers to the release number for the given year and month, starting from `0` and incrementing by one for every release.

Here is an example of a theorhetical first release in January 2025:

```
2025.1.0
```

Here is an example of a theorhetical third release in December 2024:

```
2024.12.2
```

In both examples, the exact day of release did not matter.

## `2025.4.0` - Wed 02 Apr 2025

- Add many new pre-built binaries, including macOS x86_64, Linux (GNU) aarch64, Linux (GNU) powerpc64le, and Linux (MUSL) aarch64
- Update dependencies

## `2025.2.1` - Tue 27 Feb 2025

- Add MUSL pre-built binaries
- Fix release builds for all platforms
- Yank previous release due to broken release builds

## `2025.2.0` (yanked) - Tue 27 Feb 2025

- Add "paths" configuration option to allow for multiple paths for `gfold` to execute on from [@uncenter](https://github.com/uncenter)
- Move logging verbosity from an environment variable to a flag
- Deprecate "path" configuration option from [@uncenter](https://github.com/uncenter)
- Polish help message, including its formatting
- Remove unused `strum` dependency
- Slightly reduce binary size by no longer relying on formal error types and unneeded abstractions from a multi-crate workspace (i.e. the repository now contains only one crate, yet again)
- Support `~` and `$HOME` for "paths" configuration option from [@uncenter](https://github.com/uncenter)
- Switch to "CalVer" for versioning scheme (no end user action required)
- Update dependencies

## Before `2025.2.0`

Please see [CHANGELOG_PRE_CALVER_POST_V4](./docs/CHANGELOG_PRE_CALVER_POST_V4.md).
