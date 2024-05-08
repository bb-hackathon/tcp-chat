{
    pkgs,
    ...
}:

pkgs.mkShell {
    NIX_CONFIG = "extra-experimental-features = nix-command flakes repl-flake";

    # FIXME: Does not get exported for some reason.
    PROTOC = "${pkgs.protobuf}/bin/protoc";
    PROTOC_INCLUDE = "${pkgs.protobuf}/include";

    buildInputs = with pkgs; [
        protobuf # Protocol Buffers, Google's data interchange format.
        grpcurl  # Command-line tool for interacting with gRPC servers.
        bloomrpc # GUI Client for gRPC Services (like Postman).
    ];
}
