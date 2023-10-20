WASI_SDK_PATH?=../../wasi-sdk

.PHONY: host-functions
host-functions:
	$(WASI_SDK_PATH)/bin/clang -O2 -g -o host-functions.wasm host-functions.c -mexec-model=reactor -Wl,--export=count_vowels -Wl,--import-undefined

# run:
# 	extism call ./count-vowels.wasm count_vowels --wasi --input "this is a test"