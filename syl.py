"""Compare syllabification (and monotonic conversion) of the different libraries."""

import greek_accentuation.syllabify as gas
import modern_greek_accentuation.syllabify as mgas
import modern_greek_accentuation.accentuation as mgac
import grac

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
    print(f"Modern:        {join(grac.syllabify_el(word))}")
    print(f"Modern (syn):  {join(grac.syllabify_el_mode(word, True))}")
    print(f"Modern (~syn): {join(grac.syllabify_el_mode(word, False))}")
    print(f"Modern (syn1): {join(grac.syllabify_el_mode_at(word, [1]))}")
    print(f"Monotonic:     {grac.to_mono(word)}")


if __name__ == "__main__":
    main()
