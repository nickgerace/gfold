# Release

This document contains all information related to release.

## Checklist

This checklist details the `gfold` release process.

### Preparation

- [ ] Checkout (or create a branch of) `main` at its latest commit.
- [ ] Change the `version` field in `Cargo.toml` to `<tag>`.
- [ ] (Skip for release candidates) change the version in `CHANGELOG.md` and uncomment the line, `<!--The latest version contains all changes.-->`.
- [ ] Run `cargo xtask release` and verify that everything looks/works as expected.
- [ ] Create a commit with the following message: `Update to <tag>`. Do not push (or merge) the commit.
- [ ] Test and verify the publishing workflow: `cargo publish --dry-run`.
- [ ] Finally, push (or merge) the preparation commit into `main`.

### Release Time

- [ ] Once the prepation commit has been pushed (or merged) into `main`, checkout and/or update `main`.
- [ ] Tag with `git tag <tag>` and push the tag: `git push --tags origin main`.
- [ ] Now, publish the crate: `cargo publish`.

### Post Release

- [ ] Check the [crate](https://crates.io/crates/gfold) on `crates.io`.
- [ ] Check the [docs](https://docs.rs/gfold) on `docs.rs`.
- [ ] Download the crate via `cargo install gfold` or `cargo install --version <tag> gfold`
- [ ] Check the [release](https://github.com/nickgerace/gfold/releases) on the repository's releases page.

### Update the Homebrew Tap

- [ ] Update the formula for the [tap](https://github.com/nickgerace/homebrew-nickgerace).
- [ ] Update the formula for the deprecated [tap](https://github.com/nickgerace/homebrew-gfold).
