# Developing

This document contains all tips, tricks and notes related to developing `gfold` and `gfld`.

## Building `gfld`

Since `gfld` is the minimal version of `gfold`, we need to slim it down for general testing and usage.
On a compatible platform, execute the following commands:

```sh
TEMP_TARGET=$(pwd)
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo build --bin gfld --release
strip target/release/gfld
du -h target/release/gfld | cut -f -1
time target/release/gfld $TEMP_TARGET
```

*Note: you can change `TEMP_TARGET` to a directory you'd like to target.*