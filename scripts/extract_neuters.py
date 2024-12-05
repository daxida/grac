"""Write a list with all the neuter nouns ending in iota.

Actually, it selects words that end in iota such that 'word + α'
is also in the dictionary. Turns out to be quite precise.

There is also an extensive list here:
https://el.wiktionary.org/wiki/Κατηγορία:Ουσιαστικά_που_κλίνονται_όπως_το_%27τραγούδι%27_(νέα_ελληνικά)
"""

import re
from pathlib import Path

from grac import syllabify_el

VOWEL_ACCENTED = re.compile(r"[έόίύάήώ]")

# Available here (iso-8859-7):
# http://www.elspell.gr/
# Also here (utf-8):
# https://github.com/ONLYOFFICE/dictionaries/tree/master/el_GR
ppath = Path("scripts")
dic_path = ppath / "el_GR.dic"
output_path = ppath / "neuters.txt"


def load_words() -> list[str]:
    encodings = ["iso-8859-7", "utf-8"]
    for encoding in encodings:
        try:
            with dic_path.open("r", encoding=encoding) as dic_file:
                dic_file.readline()
                return dic_file.read().splitlines()
        except (UnicodeDecodeError, FileNotFoundError):
            continue

    raise RuntimeError(
        "Unable to decode the file with the provided encodings: iso-8859-7, utf-8"
    )


def filter_neuter(words: list[str]) -> list[str]:
    words_set = set(words)
    neuter_words = []
    for word in words:
        if word[0].isupper():
            continue
        if len(word) < 2:
            continue
        if word[-1] != "ι":
            continue
        if word[-2] in "αεηιου":
            continue
        plural = word + "α"
        if plural not in words_set:
            continue
        syllables = syllabify_el(plural)
        if len(syllables) < 3 or not VOWEL_ACCENTED.search(syllables[-3]):
            continue
        neuter_words.append(plural)

    # https://el.wiktionary.org/wiki/Παράρτημα:Ουσιαστικά_(νέα_ελληνικά)/ουδέτερα#-υ_ουδέτερα
    neuter_words.extend(["βράδια", "δάκρυα", "δίκτυα", "δίχτυα", "στάχυα"])
    neuter_words.sort()

    return neuter_words


def main() -> None:
    words = load_words()
    neuter_words = filter_neuter(words)
    with output_path.open("w", encoding="utf-8") as of:
        of.writelines(line + "\n" for line in neuter_words)
    print(f"Succesfully wrote {len(neuter_words)} neuter words at {output_path}")


if __name__ == "__main__":
    main()
