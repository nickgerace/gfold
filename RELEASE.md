# Release

This document contains all information related to release.
Currently, both `gfold` and `gfld` share the same semver, which is not ideal, but practical for keeping both projects in one repository.

## Preparation

Run each step in both subcrates.

- Change the version in `Cargo.toml` to the `<new-tag>` for both `gfold` and `gfld`.
- Run the commands and verify that everything looks/works as expected:

```sh
cargo update
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test -- --nocapture
cargo doc --open
cargo build --release
```

- Change the version in `CHANGELOG.md` and uncomment the line, `<!--The latest version contains all changes.-->`.
- Create a commit with the following message: `Update to <new-tag>`. Do not push (or merge) the commit.
- Test the publishing workflow within each crate:

```sh
( cd gfold; cargo publish --dry-run )
( cd gfld; cargo publish --dry-run )
```

Finally, push (or merge) the preparation commit.

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

## Updating the Homebrew Taps

Update the formula for each tap.

- [gfold tap](https://github.com/nickgerace/homebrew-gfold).
- ~~gfld tap~~ *(coming soon)*
