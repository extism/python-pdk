#!/usr/bin/env bash

PYO3_NO_PYTHON=1 cargo build --target=wasm32-wasi --release

set -x

extism call \
    target/wasm32-wasi/release/py-func-caller.wasm \
    _start \
    --allow-path $(pwd)/target/wasm32-wasi/wasi-deps/usr:/usr \
    --input="Benjamin" \
    --wasi

set +x
