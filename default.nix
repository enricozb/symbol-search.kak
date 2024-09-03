{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "kak-symbol-search";
  version = "0.4.4";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };
}
