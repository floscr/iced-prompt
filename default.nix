{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, lib ? stdenv.lib
  # A set providing `buildRustPackage :: attrsets -> derivation`
, rustPlatform ? pkgs.rustPlatform
, fetchFromGitHub ? pkgs.fetchFromGitHub
, gitignoreSrc ? null
, pkgconfig ? pkgs.pkgconfig
, gtk3 ? pkgs.gtk3
, glib ? pkgs.glib
, gobject-introspection ? pkgs.gobject-introspection
}:

let
  gitignoreSource =
    if gitignoreSrc != null
    then gitignoreSrc.gitignoreSource
    else (import
      (fetchFromGitHub {
        owner = "hercules-ci";
        repo = "gitignore";
        rev = "c4662e662462e7bf3c2a968483478a665d00e717";
        sha256 = "0jx2x49p438ap6psy8513mc1nnpinmhm8ps0a4ngfms9jmvwrlbi";
      })
      { inherit lib; }).gitignoreSource;
in
rustPlatform.buildRustPackage rec {
  pname = "sample-flake-rust";
  version = "0.0.1";

  src = gitignoreSource ./.;

  buildInputs = with pkgs; [
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
  nativeBuildInputs = [ pkgconfig ];

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "iced-0.9.0" = "sha256-z/tkUdFXNjxR5Si8dnNrkrvFos0VAqGjnFNSs88D/5w=";
      "winit-0.28.6" = "sha256-szB1LCOPmPqhZNIWbeO8JMfRMcMRr0+Ze0f4uqyR8AE=";
    };
  };

  meta = with stdenv.lib; {
    homepage = "";
    description = "Sample flake repository for a Rust application";
    license = licenses.mit;
  };
}
