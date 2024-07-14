MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

.DEFAULT_GOAL:=test

test:
	cd $(MAKEPATH)/lib/libgfold-v5; cargo test --release -- --nocapture
	cd $(MAKEPATH)/lib/libgfold; cargo test --release -- --nocapture
.PHONY: test
