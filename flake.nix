{
  description = "A beginning of an awesome project bootstrapped with github:bleur-org/templates";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
  # @ inputs
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      formatter = pkgs.nixfmt-tree;
      devShells.default = import ./shell.nix {inherit pkgs;};
      packages.default = pkgs.callPackage ./. {inherit pkgs;};
    });

}
