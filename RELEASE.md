# Release

This document contains all information related to release.
Currently, both `gfold` and `gfld` share the same semver, which is not ideal, but practical for keeping both projects in one repository.

## Preparation

Run each step in both subcrates.

1. Change the version in `Cargo.toml` to the `<new-tag>` for both `gfold` and `gfld`.
1. Run the commands below this list and verify that everything looks/works as expected.
1. Change the version in `CHANGELOG.md` and uncomment the line, `<!--The latest version contains all changes.-->`.
1. Create a commit with the following message: `Update to <new-tag>`. Do not push (or merge) the commit.
1. Test the publishing workflow within each crate: `cargo publish --dry-run`.
1. Push (or merge) the preparation commit.

```sh
cargo update
cargo fmt
cargo clippy
cargo test -- --nocapture
cargo doc --open
cargo build --release
```

## Tagging and Publishing

Once the prepation commit has been pushed (or merged) into `main`, execute the following commands:

```sh
git tag <new-tag>
git push --tags origin main
```

Now, publish each crate.

```sh
( cd gfold; cargo publish )
( cd gfld; cargo publish )
```

Check `crates.io` and `docs.rs` afterwards:

- [gfold crate](https://crates.io/crates/gfold)
- [gfld crate](https://crates.io/crates/gfld) 

## Updating the Tap (`gfold` only)

Update the formula for the [tap](https://github.com/nickgerace/homebrew-gfold).

## Edit the Release (`gfold` only)

Check the release description and edit as necessary.

```sh
https://github.com/nickgerace/gfold/releases/tag/<new-tag>
```