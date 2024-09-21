{pkgs ? import <nixpkgs> {}}:

let 
    targets = {
        x86_64 = "x86_64-unknown-linux-musl";
        arm64 = "aarch64-unknown-linux-musl";
    };
    fpm_types = ["rpm" "apk"];
    base_packages = builtins.mapAttrs (
        name: triple: 
        let pkgs = import <nixpkgs> {crossSystem.config = triple;}; in
        pkgs.pkgsStatic.callPackage ../default.nix {}
    ) targets;
    deb_packages = builtins.mapAttrs (
        name: triple:
        pkgs.callPackage ./debian.nix {} {package = builtins.getAttr name base_packages;}
    ) targets;
    fpm_packages = builtins.listToAttrs (
        map (
            type: 
            {
                name = type;
                value = builtins.mapAttrs (
                    name: deb_package:
                    pkgs.callPackage ./fpm.nix {} {inherit deb_package type;}
                ) deb_packages;
            }
        ) fpm_types
    );
in
fpm_packages // {
    bin = base_packages;
    deb = deb_packages;
}
