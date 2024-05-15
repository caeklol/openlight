{ pkgs, lib, config, inputs, ... }:
let
	libs = with pkgs; [
		xorg.libX11
		libxkbcommon

		libGL
		libGLU

		xorg.libxcb
		xorg.libXcursor
		xorg.libXrandr
		xorg.libXi

    	xorg.libXrender

		openssl.dev
		fontconfig.lib

		gtk3.dev
		gtk3-x11.dev
	];
in
{
	env.LD_LIBRARY_PATH = lib.makeLibraryPath libs;
	
	packages = with pkgs; [
		pkg-config
		cmake
	] ++ libs;

	enterShell = ''
		export SHELL=${pkgs.bashInteractive}/bin/bash
	'';
	
	languages.rust = {
		enable = true;
		channel = "stable";
	};
}
