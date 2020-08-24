# gfold
# https://nickgerace.dev
#
# Note: this Makefile is not required to work with this repository.
# It is an optional helper.

MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NAME:=gfold
VERSION:=0.3.0

run: fmt
	@cd $(MAKEPATH); cargo run -- -p ..

build: fmt
	@cd $(MAKEPATH); cargo build

fmt:
	@cd $(MAKEPATH); cargo fmt

tree:
	cd $(MAKEPATH); cargo tree

static:
	docker pull clux/muslrust
	cd $(MAKEPATH); docker run -v $(MAKEPATH):/volume --rm -t clux/muslrust cargo build --release

version:
	grep -r --exclude-dir=target $(VERSION) $(MAKEPATH)
