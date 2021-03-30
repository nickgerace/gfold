# Developing

This document contains all tips, tricks and notes related to developing `gfold` and `gfld`.

## Building `gfld`

Since `gfld` is the minimal version of `gfold`, we need to slim it down for general testing and usage.
On a compatible platform, execute the following commands:

```sh
cargo fmt
cargo clippy
cargo build --release
strip target/release/gfld
du -h target/release/gfld | cut -f -1
```