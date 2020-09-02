# gfold
# https://nickgerace.dev
#
# Note: this Makefile is not required to work with this repository.
# It is an optional helper.

MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NAME:=gfold
VERSION:=0.4.0

run:
	@cd $(MAKEPATH); cargo run -- ..

run-recursive:
	@cd $(MAKEPATH); cargo run -- .. -r

install:
	cargo install --git https://github.com/nickgerace/gfold --tag $(VERSION)

install-local:
	cargo install --path $(MAKEPATH)

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

prepare-release:
	@printf "Change version at the following locations...\n"
	@printf "    Makefile\n    README.md\n    main.rs\n"
	@printf "Then, run the following command...\n"
	@printf "    time make build-release\n"

tag:
	cd $(MAKEPATH); git tag $(VERSION)
	cd $(MAKEPATH); git push --tags origin master

fixme:
	@cd $(MAKEPATH); grep -r \
		--exclude-dir={target,.git} \
		--exclude=Cargo.lock \
		--exclude=CHANGELOG.md \
		--color=always \
		FIXME $(MAKEPATH)
