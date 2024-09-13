build:
	./build.py build

install:
	./build.py install

format:
	uv run ruff format lib/src/prelude.py
	uv run ruff format bin/src/invoke.py
	uv run ruff format build.py

check:
	uv run ruff check lib/src/prelude.py
	uv run ruff check bin/src/invoke.py
	uv run ruff check build.py

clean:
	rm -rf bin/target lib/target

core:
	cd lib && cargo build --release

test: examples
	EXTISM_ENABLE_WASI_OUTPUT=1 extism call ./examples/count-vowels.wasm count_vowels --wasi --input "this is a test"
	EXTISM_ENABLE_WASI_OUTPUT=1 extism call ./examples/imports.wasm count_vowels --wasi --input "this is a test" --link example=./examples/imports_example.wasm
	

.PHONY: examples
examples: build
	./extism-py -o examples/count-vowels.wasm examples/count-vowels.py
	./extism-py -o examples/imports.wasm examples/imports.py
	./extism-py -o examples/imports_example.wasm examples/imports_example.py
	
