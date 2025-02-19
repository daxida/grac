build:
  maturin develop --uv --release -m py-grac/Cargo.toml

lint:
  uvx ruff check --output-format=concise
