{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # We want to use packages from the binary cache
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = { url = "github:hercules-ci/gitignore.nix"; flake = false; };
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ]
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          gitignoreSrc = pkgs.callPackage inputs.gitignore { };
          toolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rustfmt" "clippy" "rust-src" ];
          };
        in
        rec {
          packages.hello = pkgs.callPackage ./default.nix { inherit gitignoreSrc; };

          legacyPackages = packages;

          defaultPackage = packages.hello;

          devShell =
            let
              deps = with pkgs; [
                evcxr
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
                libGL
                freetype
                pkg-config
                freetype.dev
                expat

                gtk3
                glib
                gobject-introspection

                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
                libGL
                freetype
                pkg-config
                freetype.dev
                expat

              ];
            in
            pkgs.mkShell rec
            {
              CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

              LD_LIBRARY_PATH =
                builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" deps;

              nativeBuildInputs = with pkgs; [
                pkg-config
              ];
              RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";

              buildInputs = with pkgs;
                [
                  pkgs.rust-analyzer-unwrapped
                  toolchain
                ] ++ deps;
            };
        });
}
