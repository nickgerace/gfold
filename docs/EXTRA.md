# Extra

This document contains extra information related to using `gfold`.

## Post-Installation

It is highly recommended to run `strip` against the binary on compatible systems to reduce executable size.

```sh
( TEMP=$(command -v gfold); du -h $TEMP; strip $TEMP; du -h $TEMP )
```

> The above script will exit with a non-zero exit code if `gfold` is not installed and/or is not in your `PATH`.

## Automatic Upgrades with Cargo

Keeping the crate up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).

```sh
cargo install cargo-update
cargo install-update -a
```

You can chain this together with the **Post-Installation** step for automatic upgrades.

## Where is `gfld`?

`gfld` was an experimental, minimal version of `gfold`.
It was intended to potentially replace `gfold`, but has since been removed.
All optimizations and lessons learned from the project have since been integrated into `gfold`.
Check out the [removal issue to learn more](https://github.com/nickgerace/gfold/issues/110).
