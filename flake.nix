{
  description = "gfold development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [
        (import rust-overlay)
      ];

      rust-version = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
      rust-bin-with-overrides = rust-version.override {
        extensions = ["rust-analyzer" "rust-src"];
      };

      pkgs = import nixpkgs {inherit overlays system;};
    in
      with pkgs; rec {
        devShells.default = mkShell {
          packages = [
            alejandra
            rust-bin-with-overrides
          ];
        };

        formatter = alejandra;
      });
}
