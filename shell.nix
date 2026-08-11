{
  pkgs ?
    let
      lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
      nixpkgs = fetchTarball {
        url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
        sha256 = lock.narHash;
      };
    in
    import nixpkgs { overlays = [ ]; },
  ...
}:
let
  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.mkShell {
  name = "${manifest.name}";

  packages = with pkgs; [
    nixd
    statix
    deadnix
    nixfmt
    nixfmt-tree
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    cargo-watch
    just
    just-formatter
    just-lsp
    openssl
    gtk4
    meson
    ninja
    pango
    gettext
    vte-gtk4
    pkg-config
    gdk-pixbuf
    libadwaita
    pkg-config
    desktop-file-utils
    wrapGAppsHook4
    rustPlatform.bindgenHook
  ];

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  PKG_CONFIG_PATH = "${pkgs.polkit.dev}/lib/pkgconfig";
}
