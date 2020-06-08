# gfold
# https://nickgerace.dev
#
# Note: this Makefile is not required to work with this repository.
# It is an optional helper.

MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NAME:=gfold

cargo-tree:
	cd $(MAKEPATH); cargo tree

build-static:
	docker pull clux/muslrust
	cd $(MAKEPATH); docker run -v $(MAKEPATH):/volume --rm -t clux/muslrust cargo build
