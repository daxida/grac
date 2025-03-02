"""Used to generate code for chars.rs."""

import unicodedata


def generate_rust_function(
    fn_name: str = "base_lower_gc",
    ignore_coptic: bool = True,
    ignore_punct: bool = False,
    to_lowercase: bool = True,
) -> str:
    if not ignore_coptic:
        greek_and_coptic = range(0x0370, 0x03FF + 1)
    else:
        greek_and_coptic = range(0x0370, 0x03CF)
    greek_extended = range(0x1F00, 0x1FFF + 1)

    base_mapping = {}

    for codepoint in greek_and_coptic:
        char = chr(codepoint)

        if ignore_punct and not unicodedata.category(char).startswith("L"):
            continue

        decomposed = unicodedata.normalize("NFD", char)
        base_char = decomposed[0]
        if to_lowercase:
            base_char = base_char.lower()

        # Skip characters that are their own base
        if base_char != char:
            if base_char not in base_mapping:
                base_mapping[base_char] = []
            base_mapping[base_char].append(char)

    # Sorting for efficiency: lowercase, then uppercase, then others
    base_mapping = dict(
        sorted(
            base_mapping.items(),
            key=lambda p: (
                not unicodedata.category(p[0]).startswith("L"),
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


if __name__ == "__main__":
    print(generate_rust_function())
