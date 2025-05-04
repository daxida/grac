"""Used to generate code for chars.rs."""

import unicodedata
from itertools import chain

GREEK_AND_COPTIC = range(0x0370, 0x03FF + 1)
GREEK_BASE = range(0x0380, 0x03CF)
"""Only the modern Greek characters in GREEK_AND_COPTIC"""
GREEK_EXTENDED = range(0x1F00, 0x1FFF + 1)
GREEK_FULL = chain(GREEK_AND_COPTIC, GREEK_EXTENDED)
GREEK = chain(GREEK_BASE, GREEK_EXTENDED)
VOWELS_LOWER = "αειουωη"


def is_punct(c: str) -> bool:
    return not unicodedata.category(c).startswith("L")


def generate_rust_function(
    fn_name: str = "base_lower_gc",
    ignore_coptic: bool = True,
    ignore_punct: bool = True,
    to_lowercase: bool = True,
) -> str:
    if ignore_coptic:
        codepoints = GREEK_BASE
    else:
        codepoints = GREEK_AND_COPTIC
    # codepoints = GREEK_EXTENDED
    # codepoints = GREEK_FULL
    # codepoints = GREEK

    base_mapping: dict[str, list[str]] = {}

    for codepoint in codepoints:
        char = chr(codepoint)

        # Skip punctuation if the flag is set
        if ignore_punct and is_punct(char):
            continue

        decomposed = unicodedata.normalize("NFD", char)
        base_char = decomposed[0]
        if to_lowercase:
            base_char = base_char.lower()

        # Skip characters that are their own base
        if base_char == char:
            continue

        if base_char not in base_mapping:
            base_mapping[base_char] = []
        base_mapping[base_char].append(char)

    # Sorting for efficiency: lowercase, then uppercase, then others
    base_mapping = dict(
        sorted(
            base_mapping.items(),
            key=lambda p: (
                is_punct(p[0]),
                not p[0].islower(),
                p[0],
            ),
        )
    )

    code = f"fn {fn_name}(ch: char) -> char {{\n    match ch {{\n"
    for base, chars in base_mapping.items():
        variations = " | ".join(f"'{ch}'" for ch in chars)
        code += f"        {variations} => '{base}',\n"
    code += "        _ => ch,\n    }\n}\n"

    return code


def main() -> None:
    print(generate_rust_function())


if __name__ == "__main__":
    main()
