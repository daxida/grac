build:
  maturin develop --uv --release -m py-grac/Cargo.toml

t:
  python3 syl.py
  python3 cmp.py
  python3 mono.py

lint:
  uvx ruff check --output-format=concise
