{pkgs
,stdenv
,lib
,writeTextFile
}:
{package}:

let
    name = package.pname;
    control = writeTextFile {
        name = "control";
        text = ''
            Package: ${name}
            Version: ${lib.strings.removePrefix "v" package.version}-1
            Architecture: any
            Description: A commandline utility for line wrapping
        '';
    }; 
in
stdenv.mkDerivation {
    pname = name + "-deb";
    version = package.version;

    nativeBuildInputs = [
        pkgs.dpkg
    ];

    dontUnpack = true;

    buildPhase = ''
        mkdir -p '${name}/bin'
        mkdir -p '${name}/DEBIAN'

        install '${package}/bin/refold' '${name}/bin/refold'
        install '${control}' '${name}/DEBIAN/control'

        dpkg-deb --root-owner-group --build '${name}'
    '';

    installPhase = ''
        mkdir -p "$out"
        cp '${name}.deb' "$out/"
    '';
}
