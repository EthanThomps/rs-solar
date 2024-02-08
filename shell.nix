{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
    rustc
    rustup
    cargo-watch
    rustfmt
  ];
}

# Development Guide 
# https://matthewrhone.dev/nixos-vscode-environment
# or curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Run nix-shell
