{
  description = "Blackjack Game";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11"; # Choose the appropriate nixpkgs version
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }: flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.blackjack = pkgs.buildRustPackage rec {
      pname = "blackjack";
      version = "1.0.0";  # Specify your version here

      src = self.path + "/blackjack";  # Point to the blackjack directory containing Cargo.toml

      # If you have any other specific build settings, configure them here
    };
  });
}
