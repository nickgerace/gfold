# Release

This document contains all information related to release.

## Checklist

This checklist details the `gfold` release process.
Steps should (and often must) be executed in sequential order.

| RC  | Release | Step                                                                                                                            |
|-----|---------|---------------------------------------------------------------------------------------------------------------------------------|
| ✔️  | ✔️      | Checkout and rebase `main` to its latest commit.                                                                                |
| ✔️  | ✔️      | Change the `version` field in `Cargo.toml` to the new tag.                                                                      |
| ⛔   | ✔️      | Change the version in `CHANGELOG.md` and uncomment the line, `<!--The latest version contains all changes.-->`.                 |
| ✔️  | ✔️      | Run `make ci build` and verify that everything looks/works as expected.                                                         |
| ✔️  | ✔️      | Create a commit with the following message: `Update to <tag>`. **Do not push (or merge) the commit yet.**                       |
| ✔️  | ✔️      | Test and verify the publishing workflow: `cargo publish --dry-run`.                                                             |
| ✔️  | ✔️      | Push (or merge) the preparation commit into `main`.                                                                             |
| ✔️  | ✔️      | Checkout and rebase `main` to its latest commit, which should be the aforementioned commit.                                     |
| ✔️  | ✔️      | Tag with `git tag <tag>` and push the tag: `git push --tags origin main`.                                                       |
| ✔️  | ✔️      | Publish the crate: `cargo publish`.                                                                                             |
| ✔️  | ✔️      | Verify that the [crate](https://crates.io/crates/gfold) on `crates.io` looks correct.                                           |
| ✔️  | ✔️      | Download the crate via `cargo install --locked gfold` or `cargo install --locked --version <tag> gfold`.                        |
| ✔️  | ✔️      | Verify that the [GitHub release](https://github.com/nickgerace/gfold/releases) on the repository's releases page looks correct. |
| ⛔   | ✔️      | Update the formula for the [Hombrew tap](https://github.com/nickgerace/homebrew-nickgerace).                                    |

## Versioning Scheme

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Generally, the versioning scheme looks like the following formats where `X` is an unsigned integer:

- **Release candidates (RCs):** `X.X.X-rc.X`
- **Full releases:** `X.X.X`