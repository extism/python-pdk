#!/usr/bin/env bash

pushd lib
cargo build --release
popd

pushd bin
cargo build --release
popd

cp bin/target/release/extism-py .
