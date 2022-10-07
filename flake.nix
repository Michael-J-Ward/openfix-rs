{
  # Modified from: https://github.com/the-nix-way/dev-templates
  # Updated to support rust-analyzer in vscode by including `rust-src`
  # as described https://github.com/the-nix-way/dev-templates/issues/4
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , flake-utils
    , nixpkgs
    , rust-overlay
    }:

    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [
        (import rust-overlay)
        (self: super: {
          rustToolchain =
            let
              rust = super.rust-bin;
              toolchain =
                if builtins.pathExists ./rust-toolchain.toml then
                  rust.fromRustupToolchainFile ./rust-toolchain.toml
                else if builtins.pathExists ./rust-toolchain then
                  rust.fromRustupToolchainFile ./rust-toolchain
                else
                  rust.stable.latest.default;
            in
            toolchain.override {
              extensions = [ "rust-src" ];
            };
        })
      ];

      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustToolchain
          # rust-src
          openssl
          pkg-config
          cargo-audit
          cargo-deny
          cargo-cross
          rust-analyzer
        ] ++ pkgs.lib.optionals (pkgs.stdenv.isLinux) (with pkgs; [ cargo-watch ]); # Currently broken on macOS

        shellHook = ''
          ${pkgs.rustToolchain}/bin/cargo --version
        '';
      };
    });
}
