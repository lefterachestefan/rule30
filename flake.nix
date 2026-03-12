{
  description = "Bevy development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};

    libPath = with pkgs;
      lib.makeLibraryPath [
        vulkan-loader
        systemd
        wayland
        libxkbcommon
        libGL
      ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        pkg-config
        alsa-lib
      ];

      buildInputs = with pkgs; [
        systemd
        wayland
        libxkbcommon
        libGL
        vulkan-loader
      ];

      LD_LIBRARY_PATH = libPath;
    };
  };
}
