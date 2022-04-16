# Developing

This document contains information related to development.

## Preparing Changes

First, update dependencies and tidy your changes.

```shell
cargo fmt
cargo update
cargo fix --edition-idioms --allow-dirty --allow-staged
cargo clippy --all-features --all-targets
```

Now, ensure that lints, tests, and builds succeed.

```shell
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo doc --all
cargo test
cargo build --all-targets
```

> Alternatively, you can replace `cargo test` above with [cargo nextest](https://github.com/nextest-rs/nextest).
>
> ```shell
> cargo nextest run
> ```

## Performance Checks

Navigate to the [README in the `scripts` directory](../scripts/README.md) for more information on
how to run performance checks.

## Optional Checks

The following checks are optional and should be run occasionally.


```shell
# This command requires a nightly toolchain to be installed.
cargo +nightly udeps
cargo bloat --release
cargo bloat --release --crates
cargo audit
```