"""Create rust code to deal with synizesis."""

import sys
from itertools import takewhile
from pathlib import Path
from typing import TextIO

from grac import remove_all_diacritics, syllabify_el_mode_at

ppath = Path("scripts/synizesis/data")


def add_endings(lemmas: list[str], endings: str) -> list[str]:
    return [f"{lemma}{ending}" for lemma in lemmas for ending in endings.split()]


def load_from_path(path: Path) -> list[str]:
    with path.open(encoding="utf-8") as f:
        return sorted(set(f.read().splitlines()))


def sort_key(word: str) -> tuple[str, str]:
    # The only reason to add the word as a second element is
    # to avoid παιδάκια / παϊδάκια from poluting the git diff
    return (remove_all_diacritics(word), word)


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

# Verb with synizesis at the last syllable.
# Ex. πιω (from πίνω)
VERBS = [
    *add_endings(["πι"], "ω εις ει ουν ες"),
    # katharevousa endings
    *add_endings(["πι"], "η ης"),
    *add_endings(["ήπι"], "α ες ε αν"),
]

# Noun (feminine) ending in ια (singular in ια, genitive in ιας).
# Ex. αρρώστια
#
# Note that if the ια form has συνίζηση so does the equivalent
# εια form, if it exists (ζήλια/εια, περιφάνια/εια)
IA_NOUN_LEMMA = [
    "αλήθει",
    "αρρώστι",
    "αρρώστει",
    "βλαστήμι",
    "γρίνι",
    "γκρίνι",
    "έγνοι",  # Always bisyl with this orthography (but έννοια can be both)
    "κατάντι",
    "κούνι",
    "φαμίλι",
    "φτώχει",
    "φτώχι",
    "περηφάνει",
    "περηφάνι",
    "πραμάτει",
    "ορφάνι",
    "ζήλει",
    "ζήλι",
    "σκούφι",
    # https://www.greek-language.gr/greekLang/modern_greek/tools/lexica/triantafyllides/search.html?lq=στεναχώρια
    "στενοχώρι",
    "στεναχώρι",
    "συμπόνι",
]
ia_noun_endings = "α ας ες"
IA_NOUN = add_endings(IA_NOUN_LEMMA, ia_noun_endings)
other_ia_noun = load_from_path(ppath / "wiki_noun_fem_ια.txt")
IA_NOUN.extend(add_endings([word[:-1] for word in other_ia_noun], ia_noun_endings))

# Adjective ending in ιος / ια / ιο.
# Ex. αλογίσιος
#
# Notes:
# * includes the pronoun ποιος (even though it has no vocative)
# * includes the pronoun τέτοιος
# * includes the ending ον for completion (archaic for adjectives,
#   but also used in modern greek for ποιος)
IOS_ADJ_LEMMA = [
    "άδει",
    "αλογίσι",
    "ασημένι",
    "αχυρένι",
    "γαλάζι",
    "γιδίσι",
    "ζαχαρένι",
    "ίσι",
    "καινούρι",
    "καινούργι",
    "νι",
]
ios_adj_endings = "ος ου ο ε οι ων ους α ας ες"
IOS_ADJ = add_endings(IOS_ADJ_LEMMA, ios_adj_endings)
other_ios_adj = load_from_path(ppath / "wiki_adj_ιος.txt")
IOS_ADJ.extend(add_endings([word[:-2] for word in other_ios_adj], ios_adj_endings))

# Pronoun ending in ιος / ια / ιο.
# Ex. ποιος
#
# Same as adjectives in ιος / ια / ιο but includes ον αν endings
IOS_PRON_LEMMA = [
    "ποι",
    "τέτοι",
]
ios_pron_endings = "ος ου ο ε οι ων ους α ας ες ον αν"
IOS_PRON = add_endings(IOS_PRON_LEMMA, ios_pron_endings)

# Noun (neuter) ending in ιο (singular in ιο, plural in ια).
# Ex. μπάνιο
IO_NOUN_LEMMA = [
    "δίκι",
    "κουράγι",
    "μπάνι",
    "ίδι",  # Ambiguous: can also be trisyl (but much more common as bisyl)
    "γέλι",
]
io_noun_endings = "ο ου α ων"
IO_NOUN = add_endings(IO_NOUN_LEMMA, io_noun_endings)
other_io_noun = load_from_path(ppath / "wiki_noun_neut_ιο.txt")
IO_NOUN.extend(add_endings([word[:-1] for word in other_io_noun], io_noun_endings))

# Noun (masculine) ending in ιος (singular in ιος, plural in ιοι).
# Ex. γιος
IOS_NOUN_LEMMA = [
    "γι",
    "γυι",  # old writing of γιος
    "ίσκι",
    "ήσκι",
    "ήλι",
    # Note: while καπετάνιος has two plurals, the one in αίοι can not take synizesis
    "καπετάνι",
]
ios_noun_endings = "ος ου ο ε οι ων ους"
IOS_NOUN = add_endings(IOS_NOUN_LEMMA, ios_noun_endings)
other_ios_noun = load_from_path(ppath / "wiki_noun_masc_ιος.txt")
IOS_NOUN.extend(add_endings([word[:-2] for word in other_ios_noun], ios_noun_endings))

# Noun (masculine) ending in ιας
# Ex. γυναικάκιας
IAS_NOUN_LEMMA = [
    "κανάγ",
    "γυναικάκ",
]
ias_noun_endings = "ιας ια ιες"
IAS_NOUN = add_endings(IAS_NOUN_LEMMA, ias_noun_endings)
other_ias_noun = load_from_path(ppath / "wiki_noun_masc_ιας.txt")
IAS_NOUN.extend(add_endings([word[:-3] for word in other_ias_noun], ias_noun_endings))

# Noun (neuter) ending in ι (singular in ι / plural in ια)
# Ex. χιόνι / χιόνια (only the plural is added)
I_IA_NOUN = []
I_IA_NOUN.extend(load_from_path(ppath / "neuters.txt"))

SYNIZESIS = [
    "βιο",
    "βιος",
    "βερεσέδια",
    "βράδια",
    "διακόσια",  # Should require a LOT of variations to be fully covered...
    "λόγια",  # Always bisyl as NOUN (can be trisyl as adj.)
    "χρόνια",
    "χούγια",
    "ψώνια",
    "μάγια",
    "σκέλια",
    # rare - cf. https://www.greek-language.gr
    "βρετίκια",
    "βρεθίκια",
    "αναδεξίμια",
    # synizesis at upsilon
    "δίχτυα",
    "στάχυα",
    "δυο",
    # Alternative spellings (the common versions should already be included)
    "γένεια",  # γένια
    "παλλικάρια",  # παλικάρια
    "μεντέρια",  # μεντέρια
    # Obsolete or wrong (yet used) spellings
    "πηρούνια",
    "σπηρούνια",
    "συντριβάνια",
    *MONOSYLLABLES,
    *VERBS,
    *IA_NOUN,
    *IOS_ADJ,
    *IOS_PRON,
    *IO_NOUN,
    *IOS_NOUN,
    *IAS_NOUN,
    *I_IA_NOUN,
]

# Words with multiple accepted pronunciations.
#
# Only contains words with accent not on the last syllable,
# and with (possible) synizesis at the last syllable.
#
# Should be manually sync with src/constants/MULTIPLE_PRONUNCIATION
MULTIPLE_PRONUNCIATION = [
    *add_endings(["άδει"], "α ας ες"),
    # For ακρίβεια, the version with synizesis should probably be prefered.
    *add_endings(["ακρίβει"], "α ας ες"),
    *add_endings(["άγι"], "ος ου ο ε οι ων ους α ας ες"),
    *add_endings(["έννοι"], "α ας ες"),
    *add_endings(["ήπι"], "α ε ες"),  # partial overlap
    *add_endings(["ίδι"], "ος ο ε οι α"),  # partial overlap
    *add_endings(["ήλι"], "ου ο"),  # partial overlap
    *add_endings(["μύρι"], "οι ων ους ες α"),
    # Others
    "αρχοντολόγια",  # takes syn if from αρχοντολόγιο, not if from αρχοντολό(γ)ι
    "πλάγια",  # takes syn if from πλάι, not if from πλάγιος
    "φυλάκια",  # takes syn if from φυλάκι, not if from φυλάκιο
    "ουράνια",  # takes syn if from (noun) ουράνια, not if from ουράνιος
]

# Not due to synizesis, but may include synizesis at some location.
#
# They are few exceptions and therefore it makes sense to just slam them
# into the synizesis.rs map, even if it is properly not that phenomenon.
#
# Does not include diminutive endings (-άκια etc.)
MERGE_AT = [
    # ai
    [add_endings(["αηδόν"], "ι ια"), [1, 3]],
    [add_endings(["αηδον"], "ιού ιών"), [1, 3]],
    [add_endings(["αηδονίσι"], ios_adj_endings), [1, 4]],
    [add_endings(["καημέν"], ios_adj_endings), [3]],
    [add_endings(["μαϊμ"], "ού ούς"), [2]],
    [add_endings(["μαϊμ"], "ούδες ούδων"), [3]],
    [add_endings(["μαϊμουδίσι"], ios_adj_endings), [1, 4]],
    [add_endings(["μαϊστράλ"], "ι ια"), [1, 3]],
    [add_endings(["νεράιδ"], "α ας ες ων"), [2]],
    [add_endings(["γάιδαρ"], "ος ου ο ε οι ων ους"), [3]],
    [add_endings(["γαϊδούρ"], "ι ια"), [1, 3]],
    [add_endings(["γαϊδουρ"], "ιού ιών"), [1, 3]],
    [add_endings(["γαϊδουρίσι"], ios_adj_endings), [1, 4]],
    [add_endings(["γαϊτάν"], "ι ια"), [1, 3]],
    [add_endings(["γαϊταν"], "ιού ιών"), [1, 3]],
    [add_endings(["χαϊβάν"], "ι ια"), [1, 3]],
    [add_endings(["χαϊβαν"], "ιού ιών"), [1, 3]],
    [add_endings(["χάιδ"], "ι ια"), [1, 2]],
    [add_endings(["χαϊδ"], "ιού ιών"), [1, 2]],
    # oi
    [add_endings(["βόιδ"], "ι ια"), [1, 2]],
    [add_endings(["βοϊδ"], "ιού ιών"), [1, 2]],
    [add_endings(["ρόιδ"], "ι ια"), [1, 2]],
    [add_endings(["ροϊδ"], "ιού ιών"), [1, 2]],
    [add_endings(["κορόιδ"], "ο ου α ων"), [2]],
    [["κοροϊδάκια"], [1, 3]],
]


def generate_lookup_synizesis(f: TextIO) -> None:
    f.write("// This was automatically generated by scripts/synizesis/build.py.\n")
    f.write("// Do not edit manually.\n\n")

    f.write("use phf::phf_map;\n\n")
    f.write(
        "static LOOKUP: phf::Map<&'static str, &'static [&'static str]> = phf_map! {\n"
    )

    mapping = {}

    for word in SYNIZESIS:
        if word in MULTIPLE_PRONUNCIATION:
            continue

        _syls = syllabify_el_mode_at(word, [1])
        syllables = str(_syls).replace("'", '"')
        mapping[word] = syllables

        _syls = [_syls[0].capitalize()] + _syls[1:]
        syllables_cap = str(_syls).replace("'", '"')
        mapping[word.capitalize()] = syllables_cap

    for words, accent_at in MERGE_AT:
        for word in words:
            _syls = syllabify_el_mode_at(word, accent_at)
            syllables = str(_syls).replace("'", '"')
            mapping[word] = syllables

            _syls = [_syls[0].capitalize()] + _syls[1:]
            syllables_cap = str(_syls).replace("'", '"')
            mapping[word.capitalize()] = syllables_cap

    for fr, to in sorted(mapping.items(), key=lambda pair: sort_key(pair[0])):
        f.write(f'    "{fr}" => &{to},\n')

    f.write("};\n\n")
    f.write(
        "pub fn lookup_synizesis(word: &str) -> Option<&'static [&'static str]> {\n"
    )
    f.write("    LOOKUP.get(word).copied()\n")
    f.write("}\n")


def generate_multiple_pronunciation_array(f: TextIO) -> None:
    documentation = r"""
# Words with multiple accepted pronunciations.
#
# Only contains words with accent not on the last syllable,
# and with (possible) synizesis at the last syllable.
""".strip()
    for line in documentation.splitlines():
        line = line.replace("#", "///")
        f.write(f"{line}\n")
    f.write("//\n")
    f.write("// This was automatically generated by scripts/synizesis/build.py.\n")
    f.write("// Do not edit manually.\n")

    f.write("#[rustfmt::skip]\n")

    capacity = 2 * len(MULTIPLE_PRONUNCIATION)
    f.write(f"pub const MULTIPLE_PRONUNCIATION: [&str; {capacity}] = [\n")
    for word in MULTIPLE_PRONUNCIATION:
        f.write(f'    "{word}", "{word.capitalize()}", \n')
    f.write("];\n")


def write_registry(path: Path) -> None:
    """Write every word (lowercase only) that contains synizesis.

    This is only used for sanity checks in git diffs.
    """
    all_words = set()
    for word in SYNIZESIS:
        if word in MULTIPLE_PRONUNCIATION:
            continue
        all_words.add(word)
    for words, _ in MERGE_AT:
        for word in words:
            all_words.add(word)

    all_words_sorted = sorted(all_words, key=sort_key)

    with path.open("w") as f:
        for word in all_words_sorted:
            f.write(f"{word}\n")


def update_constants(path: Path) -> None:
    with path.open() as f:
        lines = f.readlines()

    clean_lines = takewhile(
        lambda line: line != "/// Words with multiple accepted pronunciations.\n",
        lines,
    )

    with path.open("w") as f:
        f.writelines(clean_lines)
        generate_multiple_pronunciation_array(f)


def main() -> None:
    # generate_lookup_synizesis(sys.stdout)
    # generate_multiple_pronunciation_array(sys.stdout)

    constants_path = Path("src/constants.rs")
    update_constants(constants_path)
    print(f"Updated {constants_path}")

    registry_path = ppath / "registry.txt"
    write_registry(registry_path)
    print(f"Updated {registry_path}")

    synizesis_path = Path("src/synizesis.rs")
    with synizesis_path.open("w", encoding="utf-8") as f:
        generate_lookup_synizesis(f)
    print(f"Updated {synizesis_path}")


if __name__ == "__main__":
    main()
