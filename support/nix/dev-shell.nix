{ lib
, mkShell
, cargo
, rustc
, clippy
, rust-analyzer
, rustfmt

, terminal
}:

mkShell {
  buildInputs = [
  ]
  ++ terminal.buildInputs
  ;
  nativeBuildInputs = [
    cargo
    rustc
    clippy
    rust-analyzer
    rustfmt
  ]
  ++ terminal.nativeBuildInputs
  ;

  G_MESSAGES_DEBUG = "Terminal";
}
