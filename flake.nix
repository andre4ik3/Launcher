{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; overlays = [ fenix.overlays.default ]; };
  in {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [
        # Core modules (Rust)
        (pkgs.fenix.complete.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "rust-analyzer"
        ])

        # macOS frontend (Swift)
        swift-format
        xcodegen
      ];
    };
  });
}
