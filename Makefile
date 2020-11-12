# gfold
# https://nickgerace.dev
#
# Note: this Makefile is not required to work with this repository.
# It is an optional helper.

MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
NAME:=gfold
VERSION:=0.7.0

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

build-release: pre-build doc
	cd $(MAKEPATH); cargo build --release

pre-build:
	cd $(MAKEPATH); cargo update
	cd $(MAKEPATH); cargo fmt
	cd $(MAKEPATH); cargo clippy
	cd $(MAKEPATH); cargo test

doc:
	cd $(MAKEPATH); cargo doc --open

tag-release:
	cd $(MAKEPATH); git tag $(VERSION)
	cd $(MAKEPATH); git push --tags origin main
	cd $(MAKEPATH); cargo publish

fixme:
	@cd $(MAKEPATH); grep -r \
		--exclude-dir={target,.git} \
		--exclude=Cargo.lock \
		--exclude=CHANGELOG.md \
		--color=always \
		FIXME $(MAKEPATH)

release:
	@printf "[1] Change version at the following locations...\n"
	@printf "    Makefile:\n        $(shell grep $(VERSION) $(MAKEPATH)/Makefile)\n"
	@printf "    CHANGELOG.md:\n        $(shell grep $(VERSION) $(MAKEPATH)/CHANGELOG.md)\n"
	@printf "    Cargo.toml:\n        $(shell grep $(VERSION) $(MAKEPATH)/Cargo.toml)\n"
	@printf "[2] Uncomment the unreleased string in CHANGELOG.md...\n"
	@printf "    <!--The latest version contains all changes.-->\n"
	@printf "[3] Then, run the following command...\n"
	@printf "    time make build-release\n"
	@printf "[4] Before merging, ensure that publishing works.\n"
	@printf "    cargo publish --dry-run\n"

post-release:
	@printf "[1] Run the following command...\n"
	@printf "    time make tag-release\n"
	@printf "[2] Edit the GitHub release page for the new release.\n"
	@printf "[3] Check crates.io: https://crates.io/crates/gfold\n"
	@printf "[4] Update Homebrew tap version: https://github.com/nickgerace/homebrew-gfold\n"

