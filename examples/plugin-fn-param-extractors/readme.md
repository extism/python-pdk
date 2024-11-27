
## Building the example 

```sh
./build.py
./extism-py examples/plugin-fn-param-extractors/plugin.py -o examples/plugin-fn-param-extractors/out.wasm

```

## Calling the example

```sh
extism call examples/plugin-fn-param-extractors/out.wasm count_vowels \
   --wasi \
   --input='Hello World Test!' \
   --log-level=info \
   --config message="hello"
```