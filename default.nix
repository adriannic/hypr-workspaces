{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "hypr-workspaces";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}
