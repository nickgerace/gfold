# Release

This document contains all information related to release.

## Versioning Scheme

See the [CHANGELOG](../CHANGELOG.md) for more information.

## Checklist

Steps should be executed in sequential order.

- [ ] Checkout and rebase `main` to its latest commit, then checkout a new branch
- [ ] Change the `version` field in [`Cargo.toml`](../Cargo.toml) to the new tag
- [ ] Open a web browser tab to the following link: `https://github.com/nickgerace/gfold/compare/<last-tag>...main`
- [ ] Add a new section the version in [`CHANGELOG.md`](../CHANGELOG.md) with the current date
- [ ] Using the diff, commit messages and commit title, populate the new section with all user-relevant changes
- [ ] Once the section is finalized, determine what field should be bumped (alongside the section title) using the release version scheme
- [ ] Verify that everything looks/works as expected:

```shell
cargo xtask ci
```

- [ ] Create and _do not merge_ a commit with the following message: `Update to <tag>`
- [ ] Test and verify the publishing workflow:

```shell
cargo publish --dry-run
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
cargo publish
```

- [ ] Verify that the [crate](https://crates.io/crates/gfold) on `crates.io` looks correct
- [ ] Download and install the crate:

```shell
cargo install --locked gfold
```

- [ ] (Optional) Remove the crate:
```shell
cargo uninstall gfold
```

- [ ] Verify that the [GitHub release](https://github.com/nickgerace/gfold/releases) on the repository's releases page looks correct
