# Python PDK

> *Warning*: This is not a working component yet. Just an experiment.

This is an experiment to get python working in an Extism PDK.

There are two options I've been exploring:

+ Use [RustPython](https://github.com/RustPython/RustPython)
+ Compile CPython 3.11 to wasm, maybe adapt [SingleStore/python-wasi](https://github.com/singlestore-labs/python-wasi)

Ideally we could use CPython itself. But open to just getting something working for the time being.

## Develop

```
git clone --recurse-submodules https://github.com/extism/python-pdk.git
cd python-pdk
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
extism call  target/wasm32-wasi/debug/python_pdk.wasm eval --input="1 + 2" --wasi
```
