# Developing

This document contains all tips, tricks and notes related to developing `gfold`.

## Building

On a compatible platform, execute the following commands:

```sh
TEMP_TARGET=$HOME
cargo update
cargo +nightly fmt
cargo clippy
cargo test
cargo build --release
TEMP_BINARY=target/release/gfold
du -h $TEMP_BINARY | cut -f -1
strip $TEMP_BINARY
du -h $TEMP_BINARY | cut -f -1
time $TEMP_BINARY $TEMP_TARGET
```

> You can change `TEMP_TARGET` to a directory you'd like to target.