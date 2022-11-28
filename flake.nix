{
  description = "A very basic flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... } @ inputs:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          naersk = pkgs.callPackage inputs.naersk { };
          appBinary = import ./nix/app_binary.nix { inherit naersk; inherit pkgs; };
          dockerImage = import ./nix/docker_image.nix { inherit naersk; inherit pkgs; };

        in
        {
          packages = {
            default = dockerImage;
            inherit appBinary;
            inherit dockerImage;
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rust-bin.stable.latest.default
              diesel-cli

              pkg-config
              openssl
              cargo-insta
            ];

          };
        });
}
