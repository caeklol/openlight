{ pkgs, lib, config, inputs, ... }:
{
	packages = with pkgs; [
		openssl.dev
		fontconfig.dev
		libxkbcommon
		xorg.libxcb.dev
	];

	enterShell = ''
		export SHELL=${pkgs.bashInteractive}/bin/bash
	'';
	
	languages.rust = {
		enable = true;
		channel = "stable";
	};
}
