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
cargo build --all-targets
cargo clippy -- -D warnings
RUSTDOCFLAGS="-Dwarnings" cargo doc --all --no-deps
cargo test -- --nocapture
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

## Nix Flake (macOS and Linux only)

If you prefer to avoid mutating your local environment, you can use the `nix` [flake](./flake.nix)!

> If you do not have `nix` installed and are unsure where to start, you can check out the
> [Zero to Nix installation guide](https://zero-to-nix.com/start/install).

You can enter a `nix` environment with everything you need.

```shell
nix develop
```

If you prefer to not enter the environment and run a single command, you can use the
`--command` flag.

```shell
nix develop --command cargo test
```

Update dependencies with the following command:

```shell
nix flake update
```
