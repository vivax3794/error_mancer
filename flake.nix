{
  description = "Dev shell for Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
            nativeBuildInputs = [
               pkg-config
            ];

            buildInputs = [
                ( rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
                    extensions = ["rust-analyzer" "rust-src"];
                }) )

                mold
                just
                cargo-nextest
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
    }
    );
}

