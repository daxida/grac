"""Write a list with all the neuter nouns ending in iota.

Actually, it selects words that end in iota such that 'word + α'
is also in the dictionary. Turns out to be quite precise.

There is also an extensive list here:
https://el.wiktionary.org/wiki/Κατηγορία:Ουσιαστικά_που_κλίνονται_όπως_το_%27τραγούδι%27_(νέα_ελληνικά)
"""

from pathlib import Path

from grac import syllabify_with_merge
from grac import has_diacritic, Diacritic

# Available here (iso-8859-7):
# http://www.elspell.gr/
# Also here (utf-8):
# https://github.com/ONLYOFFICE/dictionaries/tree/master/el_GR
ppath = Path("scripts/synizesis/data")
dic_path = ppath / "el_GR.dic"
output_path = ppath / "neuters.txt"


def load_words() -> list[str]:
    encodings = ["iso-8859-7", "utf-8"]
    for encoding in encodings:
        try:
            with dic_path.open("r", encoding=encoding) as dic_file:
                dic_file.readline()
                return dic_file.read().splitlines()
        except FileNotFoundError:
            raise
        except UnicodeDecodeError:
            pass

    raise RuntimeError(
        "Unable to decode the file with the provided encodings: iso-8859-7, utf-8"
    )


def has_acute_at(syllables: list[str], pos: int) -> bool:
    return len(syllables) >= pos and has_diacritic(
        syllables[-pos], Diacritic.ACUTE.value
    )


def is_proparoxytone(word: str) -> bool:
    syllables = syllabify_with_merge(word, merge=False)
    # Although the accent can only fall on the last three syllables, our syllabify function
    # does not know that sometimes it has to merge diphthongs to make this hold true.
    # Therefore, for words like ρόιδια, the accent ends up falling at the -4 position.
    return has_acute_at(syllables, 3) or has_acute_at(syllables, 4)


def filter_neuter(words: list[str]) -> list[str]:
    """Extract neuter words that should carry synizesis.

    In particular:
    * We only consider words that, without synizesis, should have been proparoxytone.
    * Nouns ending in ι (singular in ι / plural in ια)
      Ex. χιόνι / χιόνια (only the plural is added)
          καΐκι / καΐκια
          ρολόι / ρολόγια
    """
    words_set = set(words)
    neuter_words = set()
    for word in words:
        if word[0].isupper():
            continue
        if len(word) < 2:
            continue
        if word[-1] != "ι":
            continue
        if word[-2] in "αεηιου":
            continue

        # χιόνι / χιόνια
        # χούι / χούγια
        plurals = [word + "α", word[:-1] + "για"]
        for plural in plurals:
            if plural not in words_set:
                continue
            if not is_proparoxytone(plural):
                continue
            neuter_words.add(plural)

    return sorted(neuter_words)


def main() -> None:
    words = load_words()
    neuter_words = filter_neuter(words)
    with output_path.open("w", encoding="utf-8") as of:
        of.writelines(line + "\n" for line in neuter_words)
    print(f"Succesfully wrote {len(neuter_words)} neuter words at {output_path}")


if __name__ == "__main__":
    main()
