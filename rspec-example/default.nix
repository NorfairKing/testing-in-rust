{ rustPlatform
}:
rustPlatform.buildRustPackage {
  pname = "rspec-example";
  version = "0.0.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
