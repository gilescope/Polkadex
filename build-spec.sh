# Export the local chain spec to json
cargo build --release
./target/release/polkadex-thea-node build-spec --disable-default-bootnode --chain local > customSpec.json
./target/release/polkadex-thea-node build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
cp customSpecRaw.json bootnode/
cp ./target/release/polkadex-thea-node bootnode/polkadex-thea-node