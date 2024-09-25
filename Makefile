PYTHON_FILES=lib/src/prelude.py bin/src/invoke.py build.py

build:
	./build.py build

install:
	./build.py install

format:
	uv run ruff format $(PYTHON_FILES)

check:
	uv run ruff check $(PYTHON_FILES)

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
	
