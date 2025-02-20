{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  nixConfig = {
    extra-substituters = [ "https://nix-community.cachix.org" ];
    extra-trusted-public-keys = [ "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs=" ];
  };

  outputs = { nixpkgs, flake-utils, fenix, crane, ... }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; overlays = [ fenix.overlays.default ]; };
    inherit (pkgs) lib;

    toolchain = (pkgs.fenix.complete.withComponents [
      "cargo"
      "clippy"
      "llvm-tools"
      "rust-src"
      "rustc"
      "rustfmt"
      "rust-analyzer"
    ]);

    craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
    src = craneLib.cleanCargoSource ./.;

    commonArgs = {
      inherit src;
      strictDeps = true;
      nativeBuildInputs = with pkgs; [ pkg-config ];
      buildInputs = with pkgs; [
        # Add additional build inputs here
        llvmPackages.clangWithLibcAndBasicRtAndLibcxx
        glib
        dbus
        gtk4
        libadwaita
      ] ++ lib.optionals stdenv.isDarwin [
        # Additional darwin specific inputs can be set here
        pkgs.libiconv
      ];

      # Additional environment variables can be set directly
      # MY_CUSTOM_VAR = "some value";
    };

    # Build *just* the cargo dependencies (of the entire workspace),
    # so we can reuse all of that work (e.g. via cachix) when running in CI
    # It is *highly* recommended to use something like cargo-hakari to avoid
    # cache misses when building individual top-level-crates
    cargoArtifacts = craneLib.buildDepsOnly commonArgs;

    individualCrateArgs = commonArgs // {
      inherit cargoArtifacts;
      inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
    };

    fileSetForCrate = { crate, extras ? [] }: lib.fileset.toSource {
      root = ./.;
      fileset = lib.fileset.unions ([
        ./Cargo.toml
        ./Cargo.lock
        (craneLib.fileset.commonCargoSources ./modules)
        (craneLib.fileset.commonCargoSources ./core)
        (craneLib.fileset.commonCargoSources crate)
      ] ++ extras);
    };
  in rec {
    packages = {
      default = packages.launcher-cli;
      launcher-cli = craneLib.buildPackage (individualCrateArgs // {
        pname = "launcher-cli";
        cargoExtraArgs = "-p mc";
        src = fileSetForCrate { crate = ./apps/cli; };
      });
      launcher-linux = craneLib.buildPackage (individualCrateArgs // {
        pname = "launcher-linux";
        cargoExtraArgs = "-p launcher-linux";
        src = fileSetForCrate { crate = ./apps/linux; extras = [ ./apps/linux/resources ]; };
        nativeBuildInputs = with pkgs; [
          pkg-config
          glib
          dbus
          gtk4
          libadwaita
        ];
        buildInputs = with pkgs; [
          glib
          dbus
          gtk4
          libadwaita
        ];
      });
      silo = craneLib.buildPackage (individualCrateArgs // {
        pname = "silo";
        cargoExtraArgs = "-p silo";
        src = fileSetForCrate { crate = ./apps/silo; };
      });
    };

    checks = {
      clippy = craneLib.cargoClippy (commonArgs // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets";
      });

      fmt = craneLib.cargoFmt { inherit src; };
      deny = craneLib.cargoDeny { inherit src; };
    };

    apps = {
      default = apps.launcher-cli;
      mc = apps.launcher-cli;

      launcher-cli = flake-utils.lib.mkApp {
        name = "mc";
        drv = packages.launcher-cli;
      };

      launcher-linux = flake-utils.lib.mkApp {
        drv = packages.launcher-linux;
      };

      silo = flake-utils.lib.mkApp {
        drv = packages.silo;
      };
    };

    devShells.default = craneLib.devShell {
      packages = with pkgs; [
        # Core modules (Rust)
        cargo-deny
        cargo-hakari
        pkg-config
        glib
        dbus
        gtk4
        libadwaita
      ] ++ (lib.optionals hostPlatform.isLinux [
      ]) ++ (lib.optionals hostPlatform.isDarwin [
        swift-format
        xcodegen
      ]);
    };
  });
}
