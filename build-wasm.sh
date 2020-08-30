#!/bin/sh

# only needed until we have the new version of erdpy

cd lvl1/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/band_bridge_lvl1_wasm.wasm output/band_bridge_lvl1.wasm
cd ..

cd lvl2/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/band_bridge_lvl2_wasm.wasm output/band_bridge_lvl2.wasm
cd ..
