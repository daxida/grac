"""Compare syllabify implementations to test accuracy and performance.

To test monotonic implementations use mono.py.
"""

import time
from pathlib import Path
from typing import Callable

from modern_greek_accentuation.syllabify import modern_greek_syllabify as msyl1
from grac import syllabify as msyl2

# IPATH = Path("tests/fixtures/dump.txt")
IPATH = Path("scripts/synizesis/data/el_GR.dic")

Syllables = list[str]
Fn = Callable[[str], Syllables]


def timeit(
    fn: Fn,
    words: list[str],
    version: str,
    ref_elapsed: float = 0.0,
) -> tuple[list[Syllables], float]:
    """Measure first without allocation, then recompute it to store it."""
    start_time = time.time()
    for word in words:
        fn(word)
    elapsed = time.time() - start_time
    res = [fn(word) for word in words]
    delta_str = ""
    if ref_elapsed:
        delta = 100 * (ref_elapsed - elapsed) / ref_elapsed
        delta_str = f"[Δ={delta:.2f}%]"
    print(f"syllabify{version} took {elapsed:.4f}s {delta_str}")
    return res, elapsed


def split_words(text: str) -> list[str]:
    words = []
    for line in text.splitlines():
        words.extend(line.split(" "))
    return words


def print_rust_test(word: str, syllables: Syllables) -> None:
    print(f'    ["{word}", "{"-".join(syllables)}"],')


def main() -> None:
    start_time = time.time()

    text = IPATH.read_text()

    n_diffs = 0

    for times in (1,):
        cur_text = text * times
        words = split_words(cur_text)
        print(f"Testing with {len(words)} words")

        mref, mel1 = timeit(msyl1, words, "Py")
        mres1, _ = timeit(msyl2, words, "1", mel1)

        for a, b, w in zip(mref, mres1, words):
            if a != b and b:
                print(f"{a} {b} '{w}'")
                print_rust_test(w, a)
                n_diffs += 1
                # assert False

    print(f"number of diffs: {n_diffs}")
    elapsed = time.time() - start_time
    print(f"main took {elapsed:.4f}s")


if __name__ == "__main__":
    main()
