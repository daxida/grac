build:
  maturin develop --uv --release -m py-grac/Cargo.toml

t:
  python3 syl.py
  python3 cmp.py
  python3 mono.py

lint:
  uvx ruff check --output-format=concise

clippy *args:
  cargo clippy {{args}} --all-targets --all-features -- -W clippy::nursery -W clippy::pedantic -A clippy::must_use_candidate -A clippy::module_name_repetitions -A clippy::cast_precision_loss

