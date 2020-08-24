# gfold
# https://nickgerace.dev
#
# Note: this Makefile is not required to work with this repository.
# It is an optional helper.

MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NAME:=gfold

run:
	@cd $(MAKEPATH); cargo fmt
	@cd $(MAKEPATH); cargo run -- -p ..

tree:
	cd $(MAKEPATH); cargo tree

static:
	docker pull clux/muslrust
	cd $(MAKEPATH); docker run -v $(MAKEPATH):/volume --rm -t clux/muslrust cargo build --release
