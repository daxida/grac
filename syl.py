import greek_accentuation.syllabify as gas
import modern_greek_accentuation.syllabify as mgas
import modern_greek_accentuation.accentuation as mgac
import grac

import sys

if len(sys.argv) > 1:
    word = sys.argv[1]
else:
    word = "αστειάκια"

print(f"Ancient:       {gas.syllabify(word)}")
print(f"Modern:        {mgas.modern_greek_syllabify(word)}")
print(f"Monotonic:     {mgac.convert_to_monotonic(word)}")
print("Grac")
print(f"Modern:        {grac.syllabify_el(word)}")
print(f"Modern (syn):  {grac.syllabify_el_mode(word, True)}")
print(f"Modern (~syn): {grac.syllabify_el_mode(word, False)}")
print(f"Modern (syn1): {grac.syllabify_el_mode_at(word, [1])}")
print(f"Monotonic:     {grac.to_mono(word)}")
