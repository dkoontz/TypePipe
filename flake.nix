{
  description = "";
  inputs = {
    # Forgive me.
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, flake-utils, nixpkgs, rust-overlay }@inputs: let
    shell-overlays = [
      (import rust-overlay)
      (final: prev: {
        # Use final so we can benefit from rust-overlay.
        # This will include cargo, rustfmt, and other things from Rust.
        rust-pinned = prev.rust-bin.stable.latest.default.override {
          extensions = [
            # For rust-analyzer and others.  See
            # https://nixos.wiki/wiki/Rust#Shell.nix_example for some details.
            "clippy"
            "rust-analyzer"
            "rust-src"
            "rustfmt"
          ];
        };
      })
    ];
  in (flake-utils.lib.eachDefaultSystem (system: (let
    pkgs = import nixpkgs {
      overlays = shell-overlays;
      inherit system;
    };
  in {
    devShells.default = pkgs.mkShell {
      buildInputs = [
        pkgs.cargo
        pkgs.rust-pinned
      ];
    };

    packages.default = pkgs.callPackage ./derivation.nix {};
    defaultPackage = self.packages.${system}.default;
  }))) // {
    overlays.default = final: prev: {
      repo-sync = prev.callPackage ./derivation.nix {};
    };
  };
}
