{pkgs ? import <nixpkgs> {}}:

let 
    targets = {
        x86_64 = "x86_64-unknown-linux-musl";
        arm64 = "aarch64-unknown-linux-musl";
    };
    base_packages = builtins.mapAttrs (
        name: triple: 
        let pkgs = import <nixpkgs> {crossSystem.config = triple;}; in
        pkgs.pkgsStatic.callPackage ../default.nix {}
    ) targets;
in
{
    bin = base_packages;
    deb = builtins.mapAttrs (
        name: triple:
        pkgs.callPackage ./debian.nix {} {package = builtins.getAttr name base_packages;}
    ) targets;
}
