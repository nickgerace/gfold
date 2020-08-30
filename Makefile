# gfold
# https://nickgerace.dev
#
# Note: this Makefile is not required to work with this repository.
# It is an optional helper.

MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NAME:=gfold
VERSION:=0.3.0

run:
	@cd $(MAKEPATH); cargo run -- -p ..

install:
	cargo install --git https://github.com/nickgerace/gfold

build: fmt test
	cd $(MAKEPATH); cargo build

build-static: fmt test
	docker pull clux/muslrust
	cd $(MAKEPATH); docker run -v $(MAKEPATH):/volume --rm -t clux/muslrust cargo build --release

build-release: fmt test
	cd $(MAKEPATH); cargo build --release

fmt:
	cd $(MAKEPATH); cargo fmt

test:
	cd $(MAKEPATH); cargo test

tree:
	cd $(MAKEPATH); cargo tree

grep-version:
	@cd $(MAKEPATH); grep -r \
		--exclude-dir={target,.git} \
		--exclude=Cargo.lock \
		--color=always \
		$(VERSION) $(MAKEPATH)

grep-fixme:
	@cd $(MAKEPATH); grep -r \
		--exclude-dir={target,.git} \
		--exclude=Cargo.lock \
		--color=always \
		FIXME $(MAKEPATH)
