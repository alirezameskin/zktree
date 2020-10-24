with import <nixpkgs> {};

rustPlatform.buildRustPackage {
    pname = "zktree";
    version = "0.0.1";

    src = ./.;

    cargoSha256 = "1d35jrxvhf7m04s1kh0yrfhy9j9i6qzwbw2mwapgsrcsr5vhxasn";

}
