# Developing

This document contains information related to development.

## Preparing Changes

First, update dependencies and tidy your changes.

```shell
cargo fmt
cargo update
cargo fix --edition-idioms --allow-dirty --allow-staged
cargo clippy --all-features --all-targets --no-deps
```

Now, ensure that lints, tests, and builds succeed.

```shell
cargo fmt --all -- --check
cargo clippy -- -D warnings
RUSTDOCFLAGS="-Dwarnings" cargo doc --all --no-deps
cargo test -- --nocapture
cargo build --all-targets
```

If you'd like to mass "fix" everything, you should commit/save existing work and execute the following:

```shell
cargo fix --all-targets --all-features --allow-dirty --allow-staged
cargo clippy --fix --all-features --all-targets --allow-dirty --allow-staged
```

## Running Performance Tests

See available packages with the following command:

```shell
cargo run -p
```

## Optional Checks

The following checks are optional and should be run occasionally.

```shell
# This command requires a nightly toolchain to be installed.
cargo +nightly udeps
cargo bloat --release
cargo bloat --release --crates
cargo audit
```