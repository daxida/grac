build:
  maturin develop --uv --release -m py-grac/Cargo.toml

test:
  cargo test
  cargo test --manifest-path py-grac/Cargo.toml

# delete me?
t1:
  python3 syl.py
  python3 cmp.py
  python3 mono.py

lint *args:
  uvx ruff check {{args}} --output-format=concise

clippy *args:
  cargo clippy {{args}} --all-targets --all-features -- -W clippy::nursery -W clippy::pedantic \
  -A clippy::must_use_candidate \
  -A clippy::module_name_repetitions \
  -A clippy::cast_precision_loss \
  -A clippy::unicode_not_nfc

