WASI_SDK_PATH?=../../wasi-sdk

.PHONY: globals
count-vowels:
	$(WASI_SDK_PATH)/bin/clang -O2 -g -o globals.wasm globals.c -mexec-model=reactor -Wl,--export=globals

run:
	extism call ./globals.wasm globals --loop 100
