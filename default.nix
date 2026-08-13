{ pkgs, lib, ... }:
pkgs.stdenv.mkDerivation {
  pname = "gtk-nix-rebuild";
  version = "0.1.0";

  src = ./.;

  cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
    src = ./.;
    hash = "sha256-twQc5IIgRNrn/UdGZ4aqo0gWSSMlWzzHIXCoipzqLcU=";
  };

  nativeBuildInputs = with pkgs; [
    appstream
    appstream-glib
    desktop-file-utils
    gettext
    meson
    ninja
    pkg-config
    polkit
    wrapGAppsHook4
    openssl
  ];

  buildInputs = with pkgs; [
    gtk4
    libadwaita
    desktop-file-utils
    openssl
    rustPlatform.bindgenHook
    vte-gtk4
  ];

}
