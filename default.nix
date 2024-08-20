{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "kak-symbol-search";
  version = "0.4.3";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };
}
