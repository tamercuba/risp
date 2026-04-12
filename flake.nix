{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, ... }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      devShells.x86_64-linux.default = pkgs.mkShell {
        name = "risp";
        packages = [
          pkgs.clippy
        ];
        shellHook = ''
          export NIX_SHELL_NAME="risp"
          exec zsh
        '';
      };
    };
}
