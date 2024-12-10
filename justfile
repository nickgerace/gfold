_default:
    @just --list

# Scan for potential bloat
bloat:
    cargo bloat --release
    cargo bloat --release --crates

# Build all targets
build:
    cargo build --all-targets

# Build release targets
build-release:
    cargo build --release

# Run the ci suite
ci:
    cargo fmt --all -- --check
    cargo check --all-targets --all-features --workspace
    cargo clippy --all-targets --all-features --no-deps --workspace -- -D warnings
    cargo doc --all --no-deps
    cargo test --all-targets --workspace
    cargo build --locked --all-targets

# Run update, and baseline lints and checks
prepare:
    cargo update
    cargo fmt
    cargo check --all-targets --all-features --workspace
    cargo fix --edition-idioms --allow-dirty --allow-staged
    cargo clippy --all-features --all-targets --workspace --no-deps --fix --allow-dirty --allow-staged

# Scan for vulnerabilities and unused dependencies
scan: prepare
    cargo +nightly udeps
    cargo audit

bench directory=('../'): build-release
    hyperfine --warmup 1 'target/release/gfold {{directory}}' 'gfold {{directory}}'
