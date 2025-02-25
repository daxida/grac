/// Characters used to indicate abbreviation or elision in Greek words.
///
/// Note that this does not include the dot or ellipsis, because, unlike
/// those, apostrophes can only omit one letter.
#[rustfmt::skip]
pub const APOSTROPHES: &[char] = &[
    // Three main characters described by the Unicode consortium
    // cf. https://en.wikipedia.org/wiki/Apostrophe
    // * U+0027 ' APOSTROPHE
    // * U+2019 ’ RIGHT SINGLE QUOTATION MARK
    // * U+02BC ʼ MODIFIER LETTER APOSTROPHE
    '\u{0027}', '\u{2019}', '\u{02BC}',
    // (Wrong) variants
    '᾽', '᾿', '‘'
];

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

/// Greek words with two accepted syllabifications
//
// Would need a build script / macro for inflexions
#[rustfmt::skip]
pub const ALT_SYLLABIC: [&str; 10] = [
    "ήλιος", "Ήλιος",
    "έννοια", "Έννοια",
    "ίδιος", "Ίδιος",
    "ίδιοι", "Ίδιοι",
    // With synizesis if from πίνω, without if from ήπιος
    "ήπια", "Ήπια",
];
