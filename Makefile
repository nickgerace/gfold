MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NEW:=$(MAKEPATH)/target/release/gfold
INSTALLED:=$(shell which gfold)

prepare:
	cd $(MAKEPATH); cargo update
	cd $(MAKEPATH); cargo fix --edition-idioms --allow-dirty --allow-staged
	cd $(MAKEPATH); cargo +nightly fmt
	cd $(MAKEPATH); cargo clippy --all-features --all-targets

build: ci
	cd $(MAKEPATH); cargo build --release

ci:
	cd $(MAKEPATH); cargo +nightly fmt --all -- --check
	cd $(MAKEPATH); cargo clippy -- -D warnings
	cd $(MAKEPATH); cargo test -- --nocapture

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
