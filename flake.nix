{
  description = "Blackjack Game";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";  # Adjust as needed
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }: flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.blackjack = pkgs.buildRustPackage rec {
      pname = "blackjack";
      version = "1.0.0";  # Adjust as needed

      # Now point to the root directory
      src = self.path;  # Points to the root of the repo, which now contains Cargo.toml and Cargo.lock
    };
  });
}
