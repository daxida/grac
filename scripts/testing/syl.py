"""Compare syllabification (and monotonic conversion) of the different libraries."""

import greek_accentuation.syllabify as gas
import modern_greek_accentuation.syllabify as mgas
import modern_greek_accentuation.accentuation as mgac
from grac import syllabify, syllabify_with_merge, syllabify_with_merge_at, to_monotonic

import sys


def join(syllables):
    return "-".join(syllables)


def main() -> None:
    word = "αστειάκια"
    if len(sys.argv) > 1:
        word = sys.argv[1]

    print(f"Ancient:       {join(gas.syllabify(word))}")
    print(f"Modern:        {join(mgas.modern_greek_syllabify(word))}")
    print(f"Monotonic:     {mgac.convert_to_monotonic(word)}")

    print("==========\nGrac\n==========")
    print(f"Modern:        {join(syllabify(word))}")
    print(f"Modern (syn):  {join(syllabify_with_merge(word, True))}")
    print(f"Modern (~syn): {join(syllabify_with_merge(word, False))}")
    print(f"Modern (syn1): {join(syllabify_with_merge_at(word, [1]))}")
    print(f"Monotonic:     {to_monotonic(word)}")


if __name__ == "__main__":
    main()
