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
du -h target/release/gfold | cut -f -1
strip target/release/gfold
du -h target/release/gfold | cut -f -1
time target/release/gfold $TEMP_TARGET
```

> You can change `TEMP_TARGET` to a directory you'd like to target.