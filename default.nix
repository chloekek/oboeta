{ nixpkgs ? import ./nix/nixpkgs.nix {} }:

nixpkgs.stdenvNoCC.mkDerivation {
    name = "oboeta";

    src = ./src;

    buildInputs = [
        nixpkgs.ldc
    ];

    buildPhase = ''
        find -name '*.d' -print0 | \
            xargs -0 ldc2 -O3 -dip1000 -of=覚えた
    '';

    installPhase = ''
        mkdir --parents "$out"/bin
        mv 覚えた "$out"/bin
    '';
}
