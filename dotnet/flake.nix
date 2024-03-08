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
                    dotnetCorePackages.dotnet_8.sdk
                    dotnetCorePackages.dotnet_8.runtime
                    dotnetCorePackages.dotnet_8.aspnetcore
                    roslyn-ls
                    csharp-ls
                  ];

                  nativeBuildInputs = with pkgs;
                    [
                      msbuild
                      roslyn
                    ];
                };
          });
    };
}
