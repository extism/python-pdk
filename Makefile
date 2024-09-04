build:
	bash ./build.sh

install:
	bash ./install.sh

format:
	uv run ruff format lib/src/prelude.py
	uv run ruff format bin/src/invoke.py

check:
	uv run ruff check lib/src/prelude.py
	uv run ruff check bin/src/invoke.py
