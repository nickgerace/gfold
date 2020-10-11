# gfold
# https://nickgerace.dev
#
# Note: this Makefile is not required to work with this repository.
# It is an optional helper.

MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NAME:=gfold
VERSION:=0.5.2

all: build

run:
	@cd $(MAKEPATH); cargo run -- ..

run-recursive:
	@cd $(MAKEPATH); cargo run -- .. -r

install:
	cargo install --git https://github.com/nickgerace/gfold --tag $(VERSION)

install-local:
	cargo install --path $(MAKEPATH)

build: pre-build
	cd $(MAKEPATH); cargo build

build-release: pre-build
	cd $(MAKEPATH); cargo build --release

pre-build:
	cd $(MAKEPATH); cargo fmt
	cd $(MAKEPATH); cargo clippy
	cd $(MAKEPATH); cargo test

tree:
	cd $(MAKEPATH); cargo tree

tag:
	cd $(MAKEPATH); git tag $(VERSION)
	cd $(MAKEPATH); git push --tags origin main

fixme:
	@cd $(MAKEPATH); grep -r \
		--exclude-dir={target,.git} \
		--exclude=Cargo.lock \
		--exclude=CHANGELOG.md \
		--color=always \
		FIXME $(MAKEPATH)

release:
	@printf "[1] Change version at the following locations...\n"
	@printf "    Makefile: $(shell grep $(VERSION) $(MAKEPATH)/Makefile)\n"
	@printf "    README.md: $(shell grep $(VERSION) $(MAKEPATH)/README.md)\n"
	@printf "    CHANGELOG.md: $(shell grep $(VERSION) $(MAKEPATH)/CHANGELOG.md)\n"
	@printf "    Cargo.toml: $(shell grep $(VERSION) $(MAKEPATH)/Cargo.toml)\n"
	@printf "[2] Uncomment the unreleased string in CHANGELOG.md...\n"
	@printf "    <!--The latest version contains all changes.-->\n"
	@printf "[3] Run the following command to check documentation...\n"
	@printf "    cargo doc --open\n"
	@printf "[4] Then, run the following command...\n"
	@printf "    time make build-release\n"
