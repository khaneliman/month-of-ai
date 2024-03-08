{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      genSystems = func: lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ]
        (system: func (import nixpkgs { inherit system; }));
    in
    {
      devShells = genSystems
        (pkgs:
          {
            default =
              pkgs.mkShell
                {
                  name = "month-of-ai-shell";

                  buildInputs = with pkgs; [
                    cargo
                    rustc
                    rust-analyzer
                    rustfmt
                    clippy
                    openssl
                  ]
                  ++ lib.optionals stdenv.isDarwin
                    (with pkgs.darwin.apple_sdk.frameworks;
                    [
                      SystemConfiguration
                    ]);

                  nativeBuildInputs = with pkgs;
                    [
                      pkg-config
                    ];
                };
          });
    };
}
