{
    description = "Rust development shell";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay.url = "github:oxalica/rust-overlay";
        devenv.url = "github:cachix/devenv/v0.5";
    };

    outputs = {
        self,
        nixpkgs,
        flake-utils,
        rust-overlay,
        devenv,
        ...
    }:
    flake-utils.lib.eachDefaultSystem ( system:
        let
            overlays = [ ( import rust-overlay ) ];
            pkgs = import nixpkgs {
                inherit system overlays;
            };
        in with pkgs; {

            devShells.default = mkShell {
                nativeBuildInputs = [
                    pkg-config
                ];

                buildInputs = [
                    pkg-config
                    (
                        pkgs.rust-bin.selectLatestNightlyWith (
                            toolchain: toolchain.default.override {
                                extensions = [
                                    "rust-src"
                                    "rust-analyzer"
                                ];
                                targets = [ "wasm32-unknown-unknown" ];
                            }
                        )
                    )
                    pkgs.wasm-pack
                    pkgs.cargo-generate
                    pkgs.openssl
                    pkgs.perl
                    pkgs.wasm-bindgen-cli
                ];
            };
        }
    );
}
