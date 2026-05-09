{ lib
, mkShell
, cargo
, rustc
, clippy
, rust-analyzer
, rustfmt
}:

mkShell {
  buildInputs = [
  ];

  nativeBuildInputs = [
    cargo
    rustc
    clippy
    rust-analyzer
    rustfmt
  ];
}
