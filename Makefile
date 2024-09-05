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
