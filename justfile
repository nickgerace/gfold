set unstable

gfold := canonicalize(which("gfold"))

_default:
    @just --list

# Build and run at the debug level in the parent directory
run:
    cargo run -- -vvv ..

# Build and run with the help flag
help:
    cargo run -- -h

# Scan for potential bloat (requires: cargo-bloat)
bloat:
    cargo bloat --release
    cargo bloat --release --crates

# Build all targets
build:
    cargo build --all-targets

# Build release targets
build-release:
    cargo build --release

# Run the CI suite
ci:
    cargo fmt --all -- --check
    cargo check --all-targets --all-features --workspace
    cargo clippy --all-targets --all-features --no-deps --workspace -- -D warnings
    cargo doc --all --no-deps
    cargo test --all-targets --workspace
    cargo build --locked --all-targets

# Format all code (requires: taplo)
format:
    taplo format Cargo.toml
    taplo format Cross.toml
    taplo format .cargo/config.toml
    cargo fmt

# Generate a man file and view it (requires: man)
mangen:
    cargo run -- --generate-man
    man ./gfold.1

# Update dependencies, format and run baseline lints and checks
prepare: format
    cargo update
    cargo check --all-targets --all-features --workspace
    cargo fix --edition-idioms --allow-dirty --allow-staged --workspace
    cargo clippy --all-features --all-targets --workspace --no-deps --fix --allow-dirty --allow-staged

# Upgrade dependencies, including incompatible versions (requires: cargo-edit)
upgrade:
    cargo upgrade --incompatible

# Scan for vulnerabilities (requires: cargo-audit)
audit: prepare
    cargo audit

# Scan for unused dependencies (requires: cargo-udeps, nightly rust)
udeps:
    cargo +nightly udeps

# Check which dependencies are outdated (requires: cargo-outdated)
outdated:
    cargo outdated

# Perform a loose benchmark (requires: hyperfine)
bench directory=('../'): build-release
    hyperfine --warmup 5 'target/release/gfold {{directory}}' 'gfold {{directory}}'

# Peform a release binary size comparison (requires: dua)
size: build-release
    dua target/release/gfold
    dua {{gfold}}
