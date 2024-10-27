let
  pkgs = import <nixpkgs> {};
  frameworks = pkgs.darwin.apple_sdk.frameworks;

  in pkgs.stdenv.mkDerivation {
  name = "env-rust";
  buildInputs = [
    pkgs.git
    pkgs.rustup
    pkgs.nodejs-18_x
    pkgs.cargo-generate
    pkgs.libiconv
    frameworks.Security
    frameworks.CoreFoundation
    frameworks.CoreServices
  ];

}
