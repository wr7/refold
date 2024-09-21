{pkgs, lib}:

pkgs.rustPlatform.buildRustPackage {
    pname = "refold";
    version = "v0.1.2";

    src = lib.fileset.toSource {
        root = ./.;
        fileset = lib.fileset.unions [./src ./Cargo.toml ./Cargo.lock];
    };

    cargoHash = "sha256-E7Xx1lz0OhiS5JM2ZCcaXfheCxjRDkBOUt95f256TCo=";
}
