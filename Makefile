MAKEPATH := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

.DEFAULT_GOAL := prepare

prepare:
	cd $(MAKEPATH); cargo +nightly fmt
	cd $(MAKEPATH); cargo update
	cd $(MAKEPATH); cargo fix --edition-idioms --allow-dirty --allow-staged
	cd $(MAKEPATH); cargo clippy --all-features --all-targets
.PHONY: prepare

lint:
	cd $(MAKEPATH); cargo +nightly fmt --all -- --check
	cd $(MAKEPATH); cargo clippy -- -D warnings
.PHONY: lint

test:
	cd $(MAKEPATH); cargo nextest run --success-output immediate
.PHONY: test

scan:
	cd $(MAKEPATH); cargo +nightly udeps
	cd $(MAKEPATH); cargo bloat --release
	cd $(MAKEPATH); cargo bloat --release --crates
	cd $(MAKEPATH); cargo audit
	cd $(MAKEPATH); cargo msrv
.PHONY: scan

clean:
	cd $(MAKEPATH); cargo clean
.PHONY: clean

install:
	cargo install --locked --path $(MAKEPATH)
.PHONY: install

bench-loosely:
	$(MAKEPATH)/scripts/bench-loosely.sh
.PHONY: bench-loosely

size:
	cd $(MAKEPATH); cargo build --release
	@du -h $(MAKEPATH)/target/release/gfold
.PHONY: size
