import greek_accentuation.syllabify as gas
import modern_greek_accentuation.syllabify as mgas
import modern_greek_accentuation.accentuation as mgac
import grac

import sys

word = sys.argv[1]
print(f"Ancient:      {gas.syllabify(word)}")
print(f"Modern:       {mgas.modern_greek_syllabify(word)}")
print(f"Monotonic:    {mgac.convert_to_monotonic(word)}")
print("Grac")
print(f"Modern:       {grac.syllabify_el(word)}")
print(f"Modern (syn): {grac.syllabify_el_syn(word)}")
print(f"Monotonic:    {grac.to_mono(word)}")
