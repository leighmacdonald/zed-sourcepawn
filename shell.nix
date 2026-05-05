let
  #nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.11";
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/ce657ac8a02003528e4ea4bb59d58e1c634b790c.tar.gz";

  pkgs = import nixpkgs {
    config = {
      hardeningDisable = [ "all" ];
    };
    overlays = [ ];
  };
in
pkgs.mkShell {
  shellHook = ''
    rustup default nightly
  '';
  hardeningDisable = [ "fortify" ];
  buildInputs = with pkgs; [
    rustup
    tree-sitter
    rust-analyzer
    nil
    nixd
  ];
}
