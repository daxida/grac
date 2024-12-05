"""Create rust code to deal with synizesis.

https://github.com/daxida/greek-double-accents/blob/master/greek_double_accents/constants.py
"""

from grac import syllabify_el_syn


def _add_endings(lemmas: list[str], endings: str) -> list[str]:
    return [f"{lemma}{ending}" for lemma in lemmas for ending in endings.split()]


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
I_IO_NOUN = [
    "παντζούρια",
]
# neuters_path = Path(__file__).parent / "etc/neuters.txt"
# with neuters_path.open("r", encoding="utf-8") as f:
#     I_IO_NOUN |= set(f.read().splitlines())

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
    *IA_NOUN,
    *IA_ADJ,
    *IO_IA_NOUN,
    *IO_IOS_NOUN,
    *I_IO_NOUN,
]


def generate_lookup_synizesis(f):
    f.write("pub fn lookup_synizesis(word: &str) -> Option<Vec<&str>> {\n")
    f.write("    let result = match word {\n")
    for word in sorted(SYNIZESIS):
        syllables = str(syllabify_el_syn(word)).replace("'", '"')
        f.write(f'        "{word}" => vec!{syllables},\n')
    f.write("        _ => return None,\n")  # Default case
    f.write("    };\n")
    f.write("    Some(result)\n")
    f.write("}\n")


if __name__ == "__main__":
    import sys
    from pathlib import Path

    generate_lookup_synizesis(sys.stdout)

    path = Path("src/synizesis.rs")
    with path.open("w", encoding="utf-8") as f:
        generate_lookup_synizesis(f)
