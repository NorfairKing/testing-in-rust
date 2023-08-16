{ rustPlatform
}:
rustPlatform.buildRustPackage {
  pname = "nextest-example";
  version = "0.0.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
