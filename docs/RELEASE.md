# Release

This document contains all information related to release.

## Checklist (gfold)

This checklist details the [**gfold**](../README.md) release process.
Steps should be executed in sequential order.

- [ ] Checkout and rebase `main` to its latest commit, then checkout a new branch
- [ ] Change the `version` field in [`Cargo.toml`](../bin/gfold/Cargo.toml) to the new tag
- [ ] **Full Releases Only**: change the version in [`CHANGELOG.md`](../CHANGELOG.md) and uncomment the following line: `<!--The latest version contains all changes.-->`
- [ ] Verify that everything looks/works as expected:

```shell
cargo xtask ci
```

- [ ] Create and _do not merge_ a commit with the following message: `Update to <tag>`
- [ ] Test and verify the publishing workflow:

```shell
cargo publish --dry-run -p gfold
```

- [ ] Merge the preparation commit into `main`
- [ ] Checkout and rebase `main` to its latest commit, which should be the aforementioned commit
- [ ] Tag and push the tag:

```shell
git tag <tag>
git push --tags origin main
```

- [ ] Publish the crate:

```shell
cargo publish -p gfold
```

- [ ] Verify that the [crate](https://crates.io/crates/gfold) on `crates.io` looks correct
- [ ] Download and install the crate:

```shell
# Full releases
cargo install --locked gfold

# Release candidates (RCs)
cargo install --locked --version <tag> gfold
```

- [ ] Verify that the [GitHub release](https://github.com/nickgerace/gfold/releases) on the repository's releases page looks correct
- [ ] **Full Releases Only**: Update the formula for the [Homebrew tap](https://github.com/nickgerace/homebrew-nickgerace)

## Checklist (libgfold)

This checklist details the [**libgfold**](../lib/libgfold/README.md) release process.
Steps should be executed in sequential order.

- [ ] Checkout and rebase `main` to its latest commit and checkout a new branch
- [ ] Change the `version` field in [`Cargo.toml`](../lib/libgfold/Cargo.toml) to the new tag
- [ ] **Full Releases Only**: change the version in [`CHANGELOG.md`](../lib/libgfold/CHANGELOG.md) and uncomment the following line: `<!--The latest version contains all changes.-->`
- [ ] Verify that everything looks/works as expected:

```shell
cargo xtask ci
```

- [ ] Create and _do not merge_ a commit with the following message: `Update libgfold to <tag>`
- [ ] Test and verify the publishing workflow:

```shell
cargo publish --dry-run -p libgfold
```

- [ ] Merge the preparation commit into `main`
- [ ] Publish the crate:

```shell
cargo publish -p libgfold
```

- [ ] Verify that the [crate](https://crates.io/crates/libgfol) on `crates.io` looks correct
- [ ] Ensure that the [docs](https://docs.rs/libgfold/latest/libgfold/) on `docs.rs` look correct

## Versioning Scheme

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Generally, the versioning scheme looks like the following formats where `X` is an unsigned integer:

- **Release candidates (RCs):** `X.X.X-rc.X`
- **Full releases:** `X.X.X`
