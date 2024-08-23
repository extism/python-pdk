#!/usr/bin/env bash

PYO3_NO_PYTHON=1 cargo +nightly build --target wasm32-wasi --release

set -x

EXTISM_ENABLE_WASI_OUTPUT=1 extism call \
    target/wasm32-wasi/release/py-func-caller.wasm \
    count_vowels \
    --allow-path $(pwd)/target/wasm32-wasi/wasi-deps/usr:/usr \
    --input="this is a test" \
    --log-level info \
    --wasi

set +x
