build:
  maturin develop --uv --release -m py-grac/Cargo.toml

test:
  cargo test
  cargo test --manifest-path py-grac/Cargo.toml

syl word:
  python3 scripts/testing/syl.py {{word}}

lint *args:
  uvx ruff check {{args}} --output-format=concise

clippy *args:
  cargo clippy {{args}} --all-targets --all-features -- -W clippy::nursery -W clippy::pedantic \
  -A clippy::must_use_candidate \
  -A clippy::module_name_repetitions \
  -A clippy::cast_precision_loss \
  -A clippy::unicode_not_nfc

