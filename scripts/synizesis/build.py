"""Create rust code to deal with synizesis."""

import sys
from pathlib import Path
from typing import TextIO

from grac import syllabify_el_mode_at, remove_all_diacritics

ppath = Path("scripts/synizesis/data")


def _add_endings(lemmas: list[str], endings: str) -> list[str]:
    return [f"{lemma}{ending}" for lemma in lemmas for ending in endings.split()]


MONOSYLLABLES = [
    "δια",
    "πια",
    "πλια",
    "μπλια",
    "πιο",
    "πλιο",
    "μπλιο",
    "μια",
    "μιαν",
    "μιας",
    "γεια",
    "για",
    "γιεν",
    "τζια",
]

# Verbs with synizesis at the last syllable.
# Ex. πιω (from πίνω)
VERBS = [
    *_add_endings(["πι"], "ω εις ει ουν ες"),
    *_add_endings(["ήπι"], "α ες ε αν"),
]

# Nouns ending in ια (singular in ια, genitive in ιας).
# Ex. αρρώστια
#
# Note that if the ια form has συνίζηση so does the equivalent
# εια form, if it exists (ζήλια/εια, περιφάνια/εια)
IA_NOUN_LEMMA = [
    "αλήθει",
    "αρρώστι",
    "γρίνι",
    "γκρίνι",
    "κατάντι",
    "φαμίλι",
    "φτώχει",
    "φτώχι",
    "περηφάνει",
    "περηφάνι",
    "πραμάτει",
    "ορφάνι",
    "ζήλει",
    "ζήλι",
    # https://www.greek-language.gr/greekLang/modern_greek/tools/lexica/triantafyllides/search.html?lq=στεναχώρια
    "στενοχώρι",
    "στεναχώρι",
    "συμπόνι",
]
IA_NOUN = _add_endings(IA_NOUN_LEMMA, "α ας ες")

# Adjectives ending in ιος / ια / ιο.
# Ex. αλογίσιος
# Note: includes the pronoun ποιος (even though it has no vocative)
# Note: includes the pronoun τέτοιος
# Note: includes the ending ον for completion (archaic for adjectives,
#       but also used in modern greek for ποιος)
IA_ADJ_LEMMA = [
    "αλογίσι",
    "ασημένι",
    "αχυρένι",
    "γαλάζι",
    "γιδίσι",
    "νι",
    "ποι",
    "τέτοι",
]
IA_ADJ = _add_endings(IA_ADJ_LEMMA, "ος ου ο ον ε οι ων ους α ας ες")

# Nouns ending in ιο (singular in ιο, plural in ια).
# Ex. μπάνιο
IO_IA_NOUN_LEMMA = [
    "δίκι",
    "μπάνι",
    "ίδι",  # Ambiguous: can also be trisyl (but much more common as bisyl)
    "γέλι",
]
IO_IA_NOUN = _add_endings(IO_IA_NOUN_LEMMA, "ο ου α ων")

# Nouns ending in ιος (singular in ιος, plural in ιοι).
# Ex. γιος
IOS_IOI_NOUN_LEMMA = [
    "γι",
    "ίσκι",
    "ήσκι",
    "ήλι",
    # Note: while καπετάνιος has two plurals, the one in αίοι can not take synizesis
    "καπετάνι",
]
IOS_IOI_NOUN = _add_endings(IOS_IOI_NOUN_LEMMA, "ος ου ο ε οι ων ους")

# Nouns ending in ι (singular in ι / plural in ια)
# Ex. χιόνι / χιόνια (only the plural is added)
I_IA_NOUN = []
neuters_path = ppath / "neuters.txt"
with neuters_path.open("r", encoding="utf-8") as f:
    I_IA_NOUN.extend(sorted(set(f.read().splitlines())))

SYNIZESIS = [
    "βερεσέδια",
    "βλαστήμια",
    "διακόσια",
    "λόγια",  # Always bisyl as NOUN (can be trisyl as adj.)
    "έγνοια",  # Always bisyl with this orthography (but έννοια can be both)
    "κουράγιο",
    "καινούριο",
    "καινούργιο",
    "χρόνια",
    "χούγια",
    "γένεια",  # Alternative of γένια
    # Other ια (singular)
    "ίσια",
    *MONOSYLLABLES,
    *VERBS,
    *IA_NOUN,
    *IA_ADJ,
    *IO_IA_NOUN,
    *IOS_IOI_NOUN,
    *I_IA_NOUN,
]

# Words with multiple accepted accentuations
MULTIPLE_ACCENTUATION = [
    # Should be manually sync with src/constants/MULTIPLE_ACCENTUATION
    *_add_endings(["άγι"], "ος ου ο ε οι ων ους α ας ες"),
    *_add_endings(["έννοι"], "α ας ες"),
    *_add_endings(["ήπι"], "α ε ες"),
    # Requires adapting grs logic I think
    # *_add_endings(["ίδι"], "ος ου ο ε οι ων ους α ας ες"),
    *_add_endings(["ήλι"], "ου ο"),
    # Others
    "πλάγια",  # takes syn if from πλάι, not if from πλάγιος
    "φυλάκια",  # takes syn if from φυλάκι, not if from φυλάκιο
]

# We can't just force synizesis at syllabify level on these:
SYNIZESIS_PAIRS = [
    ["βράδια", "βρά-δια"],  # we could for this one...
    ["δίχτυα", "δί-χτυα"],
    ["στάχυα", "στά-χυα"],
    ["δυο", "δυο"],
]


def generate_lookup_synizesis(f: TextIO) -> None:
    f.write("// This file was automatically generated by scripts/synizesis/build.py.\n")
    f.write("// Do not edit this file manually.\n\n")

    f.write("use phf::phf_map;\n\n")
    f.write(
        "static LOOKUP: phf::Map<&'static str, &'static [&'static str]> = phf_map! {\n"
    )

    mapping = {}

    for word in SYNIZESIS:
        if word in MULTIPLE_ACCENTUATION:
            continue

        _syls = syllabify_el_mode_at(word, [1])
        syllables = str(_syls).replace("'", '"')
        mapping[word] = syllables

        _syls = [_syls[0].capitalize()] + _syls[1:]
        syllables_cap = str(_syls).replace("'", '"')
        mapping[word.capitalize()] = syllables_cap

    for word, _syllables in SYNIZESIS_PAIRS:
        _syls = _syllables.split("-")
        syllables = str(_syls).replace("'", '"')
        mapping[word] = syllables

        syllables_cap = str([_syls[0].capitalize()] + _syls[1:]).replace("'", '"')
        mapping[word.capitalize()] = syllables_cap

    for fr, to in sorted(
        mapping.items(), key=lambda pair: remove_all_diacritics(pair[0])
    ):
        f.write(f'    "{fr}" => &{to},\n')

    f.write("};\n\n")
    f.write(
        "pub fn lookup_synizesis(word: &str) -> Option<&'static [&'static str]> {\n"
    )
    f.write("    LOOKUP.get(word).copied()\n")
    f.write("}\n")


def write_registry() -> None:
    """Write every word (lowercase only) that contains synizesis.

    This is only used for sanity checks in git diffs.
    """
    path = ppath / "registry.txt"

    all_words = []
    for word in SYNIZESIS:
        if word in MULTIPLE_ACCENTUATION:
            continue
        all_words.append(word)
    for word, _ in SYNIZESIS_PAIRS:
        all_words.append(word)

    all_words.sort(key=remove_all_diacritics)

    with path.open("w") as f:
        for word in all_words:
            f.write(f"{word}\n")


def main() -> None:
    generate_lookup_synizesis(sys.stdout)

    write_registry()

    path = Path("src/synizesis.rs")
    with path.open("w", encoding="utf-8") as f:
        generate_lookup_synizesis(f)


if __name__ == "__main__":
    main()
