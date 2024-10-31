{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "kak-symbol-search";
  version = "0.5.1";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };
}
