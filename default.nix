let
  pkgs = import <nixpkgs> {};
  frameworks = pkgs.darwin.apple_sdk.frameworks;

  in pkgs.stdenv.mkDerivation {
  name = "env-rust";
  buildInputs = [
    pkgs.git
    pkgs.rustup
    pkgs.libiconv
    frameworks.Security
    frameworks.CoreFoundation
    frameworks.CoreServices
    pkgs.wabt
  ];

}
