{
  cargo,
  darwin,
  lib,
  libgit2,
  libssh2,
  openssl,
  pkg-config,
  rustPlatform,
  stdenv,
  ...
}:
rustPlatform.buildRustPackage (let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  # It is common convention in the Nix community to use
  # "<last-release-version>-unstable-<date>" on non-release versions of a given
  # package.  Unfortunately that is impossible to do with required tools like
  # `builtins.fetchGit` because they violate flake purity.  So just stamp the
  # version from what we see in `Cargo.toml` and if "unstable" is needed, this
  # can be done by hand.
  version = cargoToml.package.version;
in {
  pname = cargoToml.package.name;
  inherit version;
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  buildInputs = [
  ];
  nativeBuildInputs = [
    cargo
    # So cargo can find our various "lib" packages.  Might not be needed.
    # Verify.
    pkg-config
  ];
})
