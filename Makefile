MAKEPATH := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

.DEFAULT_GOAL := prepare

prepare:
	cd $(MAKEPATH); cargo +nightly fmt
	cd $(MAKEPATH); cargo update
	cd $(MAKEPATH); cargo fix --edition-idioms --allow-dirty --allow-staged
	cd $(MAKEPATH); cargo clippy --all-features --all-targets
.PHONY: prepare

ci: lint test
.PHONY: ci

test:
	cd $(MAKEPATH); cargo test -- --nocapture
.PHONY: test

lint:
	cd $(MAKEPATH); cargo +nightly fmt --all -- --check
	cd $(MAKEPATH); cargo clippy -- -D warnings
.PHONY: lint

release:
	cd $(MAKEPATH); cargo build --release
.PHONY: release

build: release
.PHONY: build

clean:
	cd $(MAKEPATH); cargo clean
.PHONY: clean

install:
	cargo install --locked --path $(MAKEPATH)
.PHONY: install

scan:
	cd $(MAKEPATH); cargo +nightly udeps
	cd $(MAKEPATH); cargo bloat --release
	cd $(MAKEPATH); cargo bloat --release --crates
	cd $(MAKEPATH); cargo audit
.PHONY: scan

msrv:
	cd $(MAKEPATH); cargo msrv
.PHONY: msrv

bench-loosely:
	REPOPATH=$(MAKEPATH) $(MAKEPATH)/scripts/bench-loosely.sh
.PHONY: bench-loosely

compare: release
	@du -h $(shell which gfold)
	@du -h $(MAKEPATH)/target/release/gfold
.PHONY: compare
