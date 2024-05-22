{
    inputs,
    pkgs,
    ...
}:

(pkgs.callPackage inputs.naersk {}).buildPackage {
    src = ../..;

    nativeBuildInputs = with pkgs; [
        pkg-config
    ];

    buildInputs = with pkgs; [
        protobuf
        postgresql
    ];
}
