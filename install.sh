#!/usr/bin/env bash

pushd lib
cargo build --release
popd

pushd bin
cargo build --release
popd

cp bin/target/release/extism-py ~/.local/bin/extism-py
cp -r lib/target/wasm32-wasi/wasi-deps/usr ~/.local/share/extism-py
