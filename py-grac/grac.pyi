# https://pyo3.rs/v0.23.3/python-typing-hints.html#my_projectpyi-content

def syllabify_el(s: str) -> list[str]:
    """Extract syllables."""

def to_mono(s: str) -> str:
    """Convert polytonic to monotonic."""

def add_acute(s: str, pos: int) -> str:
    """Add acute to the one-indexed pos syllable from the end.

    Ex:
    - add_acute("ανθρωπος", 1) > "ανθρωπός"
    - add_acute("ανθρωπος", 2) > "ανθρώπος"
    """
