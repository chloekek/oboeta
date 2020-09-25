{ nixpkgs ? import ./nix/nixpkgs.nix {} }:

[
    nixpkgs.cargo
    nixpkgs.gcc
]
