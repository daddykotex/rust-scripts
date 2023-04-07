{ lib, pkgs, ... }:

{

  # https://devenv.sh/packages/
  packages = [ pkgs.git ] ++
    lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
      frameworks.CoreFoundation
      frameworks.Security
      frameworks.SystemConfiguration
    ]) ++
    lib.optionals pkgs.stdenv.isLinux ([pkgs.openssl]);

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  enterShell = ''
    git --version
  '';

  # https://devenv.sh/languages/
  # languages.nix.enable = true;

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
