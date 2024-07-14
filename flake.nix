{
  description = "gfold development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
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
      pkgs = import nixpkgs {
        inherit overlays system;
      };
    in {
      devShells.default = with pkgs;
        mkShell {
          packages = [
            alejandra
            just
          ];

          buildInputs =
            [
              (rust-bin.stable.latest.default.override {
                extensions = [
                  "rust-analyzer"
                ];
              })
            ]
            ++ lib.optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security
            ];

          formatter = alejandra;
        };
    });
}
