{
    inputs,
    pkgs,
    lib,
    ...
}:

let
    toolchain = (pkgs.rustChannelOf {
        rustToolchain = ../../rust-toolchain.toml;
        sha256 = "sha256-s1RPtyvDGJaX/BisLT+ifVfuhDT1nZkZ1NcK8sbwELM=";
    }).rust;

    naersk' = pkgs.callPackage inputs.naersk {
        cargo = toolchain;
        rustc = toolchain;
    };
in

naersk'.buildPackage rec {
    src = ../..;
    pname = "server";
    nativeBuildInputs = with pkgs; [ pkg-config ];
    buildInputs = with pkgs; [
        protobuf # Protocol Buffers, Google's data interchange format.
        grpcurl  # Command-line tool for interacting with gRPC servers.
        bloomrpc # GUI Client for gRPC Services (like Postman).
        postgresql.lib
        openssl.dev
        openssl.out
        stdenv.cc.cc.lib
    ];

    LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
