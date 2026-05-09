{ lib
, pkgs
, src ? builtins.fetchGit ../..
}:

(lib.makeScope pkgs.newScope (
  self:
  let
    inherit (self) callPackage;
    # Workspace information
    inherit
      (builtins.fromTOML (builtins.readFile (src + "/Cargo.toml")))
      workspace
    ;
  in
  {
    # Make using the CLI a bit neater.
    # This keeps `callPackage` around, as it allows using this scope semantics directly.
    self = builtins.removeAttrs self [
      "newScope"
      "overrideScope"
      "packages"
      "self"
    ];
    dev-shell = callPackage ./dev-shell.nix { };

    # Ref on the package set.
    inherit pkgs;
  }
)).self
