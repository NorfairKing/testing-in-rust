{
  description = "testing-in-rust";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-23.05";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs =
    { self
    , nixpkgs
    , pre-commit-hooks
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
        overlays = [
          self.overlays.${system}
        ];
      };
    in
    {
      overlays.${system} = import ./nix/overlay.nix;
      packages.${system} = {
        rspec-example = pkgs.rspec-example;
        nextest-example = pkgs.nextest-example;
      };
      checks.${system} = self.packages.${system} // {
        shell = self.devShells.${system}.default;
        pre-commit = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            nixpkgs-fmt.enable = true;
            tagref.enable = true;
          };
        };
      };
      devShells.${system}.default = pkgs.mkShell {
        name = "hello-world-shell";
        buildInputs = (with pkgs; [
          cargo
          cargo-nextest
          rust-analyzer
          rustc
          rustfmt
        ]) ++ (with pre-commit-hooks.packages.${system};
          [
            nixpkgs-fmt
          ]);
        shellHook = self.checks.${system}.pre-commit.shellHook;
      };
    };
}
