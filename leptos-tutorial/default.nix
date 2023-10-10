{}:
let
  rev = "2646b294a146df2781b1ca49092450e8a32814e1";
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
  pkgs = import nixpkgs { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    trunk
  ];
}
