{ lib
, mkShell
, cargo
, rustc
, clippy
, rust-analyzer
, rustfmt

# TODO: move into actual project package
, pkg-config
, gtk4
, vte-gtk4
}:

mkShell {
  buildInputs = [
    gtk4
    vte-gtk4
  ];

  nativeBuildInputs = [
    cargo
    rustc
    clippy
    rust-analyzer
    rustfmt

    pkg-config
  ];

  G_MESSAGES_DEBUG = "Terminal";
}
