{
  description = "A terminal-based Blackjack game in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = { self, nixpkgs }: 
  let
    system = "x86_64-linux";  # Change to `aarch64-linux` for ARM (e.g., Apple M1/M2)
    pkgs = import nixpkgs { inherit system; };
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      pname = "BLACKJACKCLG";
      version = "1.0.0";
      src = ./.;
      cargoLock = {
        lockFile = ./Cargo.lock;
      };
    };
    apps.${system}.default = {
       type = "app";
       program = "${self.packages.${system}.default}/bin/BLACKJACKCLG";
    };
  };
}






