{ rustPlatform
, nix-gitignore
, pkg-config
, gtk4
, vte-gtk4
}:

let
  CargoTOML = builtins.fromTOML (builtins.readFile (../../Cargo.toml));
in
rustPlatform.buildRustPackage {
  pname = CargoTOML.package.name;
  version = CargoTOML.package.version;
  src = nix-gitignore.gitignoreSource [ ] ../../.;
  cargoLock.lockFile = ../../Cargo.lock;

  buildInputs = [
    gtk4
    vte-gtk4
  ];

  nativeBuildInputs = [
    pkg-config
  ];
}
