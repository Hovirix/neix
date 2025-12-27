{
  description = "Blazing fast eix-like search tool for nixpkgs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "neix";
          version = "0.0.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          buildInputs = [ pkgs.sqlite ];

          meta = with pkgs.lib; {
            description = "Blazing fast eix-like search tool for nixpkgs";
            homepage = "https://github.com/Hovirix/neix";
            maintainers = with maintainers; [ Hovirix ];
            license = licenses.mit;
          };
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            clippy
            rustfmt
            rust-analyzer
            sqlite
          ];
        };
      });
}

