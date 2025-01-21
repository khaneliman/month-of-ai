{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          name = "month-of-ai-shell";

          buildInputs = with pkgs; [
            cargo
            rust-bin.stable.latest.default
            rustfmt
            clippy
            openssl
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };
      }
    );
}
