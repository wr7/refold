{pkgs, lib}:

pkgs.rustPlatform.buildRustPackage {
    pname = "refold";
    version = "v0.1.1";

    src = lib.fileset.toSource {
        root = ./.;
        fileset = lib.fileset.unions [./src ./Cargo.toml ./Cargo.lock];
    };

    cargoHash = "sha256-5JYFwmizdjHHsWJLrBIcHCettqYKAonAuywTVBgx+R8=";
}
