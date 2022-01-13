# Release

This document contains all information related to release.

## Checklist

This checklist details the `gfold` release process.
Steps should (and frequently must) be executed in sequential order.

- [ ] Checkout and rebase `main` to its latest commit, then checkout a new branch
- [ ] Change the `version` field in `Cargo.toml` to the new tag
- [ ] **Full Releases Only**: change the version in `CHANGELOG.md` and uncomment the following line: `<!--The latest version contains all changes.-->`
- [ ] Run final `make` targets and verify that everything looks/works as expected:

```bash
make ci build
```

- [ ] Create and _do not merge_ a commit with the following message: `Update to <tag>`
- [ ] Test and verify the publishing workflow:

```bash
cargo publish --dry-run
```

- [ ] Merge the preparation commit into `main`
- [ ] Checkout and rebase `main` to its latest commit, which should be the aforementioned commit
- [ ] Tag and push the tag:

```bash
git tag <tag>
git push --tags origin main
```

- [ ] Publish the crate:

```bash
cargo publish
```

- [ ] Verify that the [crate](https://crates.io/crates/gfold) on `crates.io` looks correct
- [ ] Download and install the crate:

```bash
# Full releases
cargo install --locked gfold

# Release candidates (RCs)
cargo install --locked --version <tag> gfold
```

- [ ] Verify that the [GitHub release](https://github.com/nickgerace/gfold/releases) on the repository's releases page looks correct
- [ ] **Full Releases Only**: Update the formula for the [Hombrew tap](https://github.com/nickgerace/homebrew-nickgerace)

## Versioning Scheme

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Generally, the versioning scheme looks like the following formats where `X` is an unsigned integer:

- **Release candidates (RCs):** `X.X.X-rc.X`
- **Full releases:** `X.X.X`