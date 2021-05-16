# Release

This document contains all information related to release.

## Preparation

- [ ] Change the `version` field in `Cargo.toml` to `<new-tag>`
- [ ] Run the commands and verify that everything looks/works as expected:

```sh
cargo update
cargo +nightly fmt --all -- --check
cargo clippy -- -D warnings
cargo test -- --nocapture
cargo doc --open
cargo build --release
```

- [ ] Change the version in `CHANGELOG.md` and uncomment the line, `<!--The latest version contains all changes.-->`.
- [ ] Create a commit with the following message: `Update to <new-tag>`. Do not push (or merge) the commit.
- [ ] Test the publishing workflow within each crate:

```sh
cargo publish --dry-run
```

- [ ] Finally, push (or merge) the preparation commit.

## Tagging and Publishing

- [ ] Once the prepation commit has been pushed (or merged) into `main`, execute the following commands:

```sh
git tag <new-tag>
git push --tags origin main
```

- [ ] Now, publish the crate.

```sh
cargo publish
```

- [ ] Check the [crate](https://crates.io/crates/gfold) on `crates.io` and `docs.rs` afterwards.

## Updating the Homebrew Tap

- [ ] Update the formula for the [tap](https://github.com/nickgerace/homebrew-gfold).
