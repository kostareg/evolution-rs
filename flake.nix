{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = {
    self,
    nixpkgs,
    devenv,
    systems,
    ...
  } @ inputs: let
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    packages = forEachSystem (system: {
      devenv-up = self.devShells.${system}.default.config.procfileScript;
      devenv-test = self.devShells.${system}.default.config.test;
    });

    devShells =
      forEachSystem
      (system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              languages.rust.enable = true;

              packages = [
                pkgs.xorg.libX11
                pkgs.xorg.libXi
                pkgs.xorg.libXcursor
                pkgs.xorg.libXrandr
                pkgs.xorg.libXinerama
                pkgs.libxkbcommon
                pkgs.xorg.libXext
                pkgs.mesa
                pkgs.mesa.drivers
                pkgs.mesa.dev
                pkgs.mesa-demos
                pkgs.libglvnd
              ];

              env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
                pkgs.xorg.libX11
                pkgs.xorg.libXi
                pkgs.xorg.libXcursor
                pkgs.xorg.libXrandr
                pkgs.xorg.libXinerama
                pkgs.libxkbcommon
                pkgs.xorg.libXext
                pkgs.mesa
                pkgs.mesa.drivers
                pkgs.libglvnd
              ];
            }
          ];
        };
      });
  };
}
