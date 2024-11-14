{ pkgs ? import <nixpkgs> {} }:

let
  hook = "";
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ rustc cargo gcc pkg-config openssl ];
  buildInputs = with pkgs; [ rustfmt clippy ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  shellHook = hook;
}
