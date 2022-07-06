# Performance Testing Scripts

Run a script by executing `cargo run` in its directory.

```shell
cd <path-to-root>/scripts/<script>/
cargo run -q
```

Alternatively, you can execute `cargo run` from another directory.
Here is an example:

```shell
cargo run -q --manifest-path <path-to-root>/scripts/<script>/Cargo.toml
```