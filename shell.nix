{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    rustfmt
    rustc
    rust-analyzer
    libiconv
    just
  ];
}

