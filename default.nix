{
# A Nixpkgs to build from, defaults to importing `nixpkgs_path`
  pkgs ? import nixpkgs_path {}
# When no `pkgs` is provided, refers to the default Nixpkgs from npins.
, nixpkgs_path ? (import ./npins).nixpkgs
# Name of the attribute used for the Nix package.
, nix ? "nix"
}:

let
  packages = 
    # This is not using `callPackage` so `override`* isn't exposed to the CLI.
    import ./support/nix/packages.nix {
      inherit (pkgs) lib;
      inherit
        pkgs
      ;
    }
  ;
in
# Expose the main package...
packages.terminal // {
  # ... while allowing access to the packages attrset.
  inherit packages;
  # ... and the dev shell.
  inherit (packages) dev-shell;
}
