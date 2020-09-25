let
    nixpkgsFile = builtins.readFile ./nixpkgs.json;
    nixpkgsSpec = builtins.fromJSON nixpkgsFile;
    nixpkgsTarball = fetchTarball nixpkgsSpec;
    config = {};
in
    {}: import nixpkgsTarball { inherit config; }
