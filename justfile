_default:
    @just --list

# Build and run at the debug level in the parent directory
run:
    cargo run -- -vvv ..

# Build and run with the help flag
run-help:
    cargo run -- -h

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

# Update the nix flake lockfile
update-flake:
    nix flake update
    
# Update deps, run formatter, and run baseline lints and checks
prepare:
    cargo update
    cargo fmt
    cargo check --all-targets --all-features --workspace
    cargo fix --edition-idioms --allow-dirty --allow-staged
    cargo clippy --all-features --all-targets --workspace --no-deps --fix --allow-dirty --allow-staged

# Scan for vulnerabilities
audit: prepare
    cargo audit

# Scan for unused dependencies (requires nightly Rust!)
udeps:
    cargo udeps

# Check which dependencies are outdated
outdated:
    cargo outdated

# Perform a loose benchmark
bench directory=('../'): build-release
    hyperfine --warmup 1 'target/release/gfold {{directory}}' 'gfold {{directory}}'

# Peform a release binary size comparison
size: build-release
    #!/usr/bin/env bash
    checker=gdu
    if ! command -v $checker; then
        checker=du
    fi
    $checker -b target/release/gfold
    binary=$(which gfold)
    if [[ -n "$binary" ]]; then
        $checker -b "$(realpath "$binary")"
    fi
