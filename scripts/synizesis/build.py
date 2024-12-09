"""Create rust code to deal with synizesis."""

from pathlib import Path

from grac import syllabify_el_mode


def _add_endings(lemmas: list[str], endings: str) -> list[str]:
    return [f"{lemma}{ending}" for lemma in lemmas for ending in endings.split()]


MONOSYLLABLES = [
    "πιο",
    "πια",
    "μια",
    "μιας",
    "για",
    "γεια",
]


# Nouns ending in ια (singular in ια, genitive in ιας).
# Ex. αρρώστια
#
# Note that if the ια form has συνίζηση so does the equivalent
# εια form, if it exists (ζήλια/εια, περιφάνια/εια)
IA_NOUN_LEMMA = [
    "αλήθει",
    "αρρώστι",
    "φτώχει",
    "φτώχι",
    "συμπόνι",
    "περηφάνει",
    "περηφάνι",
    "ορφάνι",
    "ζήλει",
    "ζήλι",
]
IA_NOUN = _add_endings(IA_NOUN_LEMMA, "α ας ες")

# Adjectives ending in ιος / ια / ιο.
# Ex. αλογίσιος
IA_ADJ_LEMMA = [
    "αλογίσι",
]
IA_ADJ = _add_endings(IA_ADJ_LEMMA, "ος ου ο ε α ας ων ους ες")

# Nouns ending in ιο (singular in ιο, plural in ια).
# Ex. μπάνιο
IO_IA_NOUN_LEMMA = [
    "δίκι",
    "μπάνι",
    "ίδι",  # Ambiguous: can also be trisyl (but much more common as bisyl)
]
IO_IA_NOUN = _add_endings(IO_IA_NOUN_LEMMA, "ο ου α ων")

# Nouns ending in ιο (singular in ιο, plural in ιος).
IO_IOS_NOUN_LEMMA = [
    "ίσκι",
]
IO_IOS_NOUN = _add_endings(IO_IOS_NOUN_LEMMA, "ος ου ο ε οι ων ους")

# Nouns ending in ι (singular in ι / plural in ια)
I_IO_NOUN = []
neuters_path = Path(__file__).parent / "neuters.txt"
with neuters_path.open("r", encoding="utf-8") as f:
    I_IO_NOUN.extend(sorted(set(f.read().splitlines())))

SYNIZESIS = [
    "λόγια",  # Always bisyl as NOUN (can be trisyl as adj.)
    "έγνοια",  # Always bisyl with this orthography (but έννοια can be both)
    "κουράγιο",
    "καινούριο",
    "καινούργιο",
    "χρόνια",
    "χούγια",
    # Other ια (singular)
    "ίσια",
    *MONOSYLLABLES,
    *IA_NOUN,
    *IA_ADJ,
    *IO_IA_NOUN,
    *IO_IOS_NOUN,
    *I_IO_NOUN,
]

# We can't just force synizesis at syllabify level on these:
SYNIZESIS_PAIRS = [
    ["βράδια", "βρά-δια"],  # we could for this one...
    ["δίχτυα", "δί-χτυα"],
    ["στάχυα", "στά-χυα"],
]


def generate_lookup_synizesis(f):
    f.write("// This file was automatically generated by scripts/synizesis/build.py.\n")
    f.write("// Do not edit this file manually.\n\n")

    f.write("use phf::phf_map;\n\n")
    f.write(
        "static LOOKUP: phf::Map<&'static str, &'static [&'static str]> = phf_map! {\n"
    )

    mapping = {}

    for word in SYNIZESIS:
        _syls = syllabify_el_mode(word, True)
        syllables = str(_syls).replace("'", '"')
        mapping[word] = syllables

        _syls = [_syls[0].capitalize()] + _syls[1:]
        syllables_cap = str(_syls).replace("'", '"')
        mapping[word.capitalize()] = syllables_cap

    for word, _syllables in SYNIZESIS_PAIRS:
        syllables = str(_syllables.split("-")).replace("'", '"')
        mapping[word] = syllables

    for fr, to in sorted(mapping.items()):
        f.write(f'    "{fr}" => &{to},\n')

    f.write("};\n\n")
    f.write(
        "pub fn lookup_synizesis(word: &str) -> Option<&'static [&'static str]> {\n"
    )
    f.write("    LOOKUP.get(word).cloned()\n")
    f.write("}\n")


if __name__ == "__main__":
    import sys
    from pathlib import Path

    generate_lookup_synizesis(sys.stdout)

    path = Path("src/synizesis.rs")
    with path.open("w", encoding="utf-8") as f:
        generate_lookup_synizesis(f)
