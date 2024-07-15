{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "kak-symbol-search";
  version = "0.3.0";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };
}
