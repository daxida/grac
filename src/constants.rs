/// Characters used to indicate abbreviation or elision in Greek words.
///
/// Note that this does not include the dot or ellipsis, because, unlike
/// those, apostrophes can only omit one letter.
pub const APOSTROPHES: &[char] = &['᾽', '᾿', '\'', '‘', '’'];

/// List of correctly accented monosyllables, excepting pronouns.
///
/// Useful to address if a word is correctly accented, since any monosyllable
/// outside of this list can be treated as an error.
pub const MONOSYLLABLE_ACCENTED: [&str; 12] = [
    "ή", "Ή", "πού", "Πού", "πώς", "Πώς", "είς", "Είς", "έν", "Έν", "έξ", "Έξ",
];

/// List of correctly accented monosyllables, including pronouns.
// TODO: Avoid duplication
#[rustfmt::skip]
pub const MONOSYLLABLE_ACCENTED_WITH_PRONOUNS: [&str; 28] = [
    "ή", "Ή", "πού", "Πού", "πώς", "Πώς", "είς", "Είς", "έν", "Έν", "έξ", "Έξ",
    // Lowercase
    "μού", "μάς", "τού", "τής", "τούς", "τών", "σού", "σάς",
    // Capitalized
    "Μού", "Μάς", "Τού", "Τής", "Τούς", "Τών", "Σού", "Σάς",
];
