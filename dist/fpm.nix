{ pkgs
, lib
, stdenv
}:

# The .deb package
{ deb_package
# Either "apk" or "rpm"
, type
}:

# Converts the .deb package into a .apk or .rpm

let
    name = lib.strings.removeSuffix "-deb" deb_package.pname;
in
stdenv.mkDerivation {
    pname = name + "-${type}";
    version = deb_package.version;

    nativeBuildInputs = [
        pkgs.fpm
        pkgs.rpm
    ];

    dontUnpack = true;

    buildPhase = ''
        mkdir -p "$out"
        fpm -s deb -t '${type}' -p "$out"'/${name}.${type}' '${deb_package}/${name}.deb' 
    '';
}
