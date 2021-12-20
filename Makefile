MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NEW:=$(MAKEPATH)/target/release/gfold
INSTALLED:=$(shell which gfold)

prepare: fmt
	cd $(MAKEPATH); cargo update
	cd $(MAKEPATH); cargo fix --edition-idioms --allow-dirty --allow-staged
	cd $(MAKEPATH); cargo clippy --all-features --all-targets
	cd $(MAKEPATH) && $(MAKE) fmt

fmt:
	cd $(MAKEPATH); cargo +nightly fmt

v3:
	@cd $(MAKEPATH); cargo run --bin v3 $(HOME)/src
.PHONY: v3

v2:
	@cd $(MAKEPATH); cargo run --bin v2 $(HOME)/src
.PHONY: v2

ci:
	cd $(MAKEPATH); cargo +nightly fmt --all -- --check
	cd $(MAKEPATH); cargo clippy -- -D warnings
	cd $(MAKEPATH); cargo test -- --nocapture

all: prepare ci

scan:
	cd $(MAKEPATH); cargo +nightly udeps
	cd $(MAKEPATH); cargo audit
	cd $(MAKEPATH); cargo bloat --release
	cd $(MAKEPATH); cargo bloat --release --crates

bench-loosely:
	@echo "============================================================="
	@time $(INSTALLED) ~/
	@echo "- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -"
	@time $(NEW) ~/
	@echo "============================================================="
	@du -h $(INSTALLED)
	@du -h $(NEW)
	@du $(INSTALLED)
	@du $(NEW)

release:
	cd $(MAKEPATH); cargo build --release
	@du -h $(MAKEPATH)/target/release/v2
	@du -h $(MAKEPATH)/target/release/v3