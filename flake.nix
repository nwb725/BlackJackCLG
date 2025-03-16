{
  description = "Blackjack Game";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";  # Adjust as needed
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }: flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.x86_64-linux.default = pkgs.buildRustPackage rec {
      pname = "blackjack";
      version = "1.0.0";  # Adjust as needed

      # Point to the correct source directory
      src = self.path;

      # Optional: specify dependencies if you have them
      nativeBuildInputs = [
        pkgs.cargo
      ];
    };

    devShell = pkgs.mkShell {
      buildInputs = [
        pkgs.rustc
        pkgs.cargo
      ];

      shellHook = ''
        export CARGO_HOME=$HOME/.cargo
        export RUSTUP_HOME=$HOME/.rustup
      '';
    };
  });
}

