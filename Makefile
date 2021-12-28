MAKEPATH := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NEW := $(MAKEPATH)/target/release/gfold
INSTALLED := $(shell which gfold)

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

scan:
	cd $(MAKEPATH); cargo +nightly udeps
	cd $(MAKEPATH); cargo bloat --release
	cd $(MAKEPATH); cargo bloat --release --crates
	cd $(MAKEPATH); cargo audit
.PHONY: scan

bench-loosely:
	@echo "============================================================="
	@time $(INSTALLED) ~/
	@echo "- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -"
	@time $(NEW) -i ~/
	@echo "============================================================="
	@du -h $(INSTALLED)
	@du -h $(NEW)
.PHONY: bench-loosely
