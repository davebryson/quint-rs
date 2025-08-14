{
  description = "Quint simulator";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.rustup
            pkgs.rustc
            pkgs.rust-analyzer
            pkgs.cargo-insta
          ];
          shellHook = ''
            rustup default 1.88
          '';
        };
      });
 }
