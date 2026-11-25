{
  description = "editrs - A highly customizable TUI text editor written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        version = cargoToml.package.version;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          inherit version;

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          buildInputs = [
            pkgs.openssl
            pkgs.zlib
            pkgs.libssh2
          ];

          nativeBuildInputs = [
            pkgs.pkg-config
          ];

          meta = with pkgs.lib; {
            description = cargoToml.package.description;
            homepage = cargoToml.package.homepage;
            license = licenses.bsd2;
            platforms = platforms.unix;
            mainProgram = cargoToml.package.name;
          };
        };

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.rustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.clippy
            pkgs.rust-analyzer
            pkgs.pkg-config

            pkgs.openssl
            pkgs.zlib
            pkgs.libssh2
          ];
        };
      })
    // {
      overlays.default = final: prev: {
        editrs = self.packages.${final.system}.default;
      };
    };
}
