/// Characters used to indicate abbreviation or elision in Greek words.
///
/// Note that this does not include the dot or ellipsis, because, unlike
/// those, apostrophes can only omit one letter.
#[rustfmt::skip]
pub const APOSTROPHES: [char; 8] = [
    // Three main characters described by the Unicode consortium
    // cf. https://en.wikipedia.org/wiki/Apostrophe
    // * U+0027 ' APOSTROPHE
    // * U+2019 ’ RIGHT SINGLE QUOTATION MARK
    // * U+02BC ʼ MODIFIER LETTER APOSTROPHE
    '\u{0027}', '\u{2019}', '\u{02BC}',

    // (Wrong) variants
    // * U+1FBD ᾽ GREEK KORONIS
    // * U+1FBF ᾿ GREEK PSILI
    // * U+2018 ‘ LEFT SINGLE QUOTATION MARK
    // * U+00B4 ´ ACUTE ACCENT
    // * U+0384 ΄ GREEK TONOS
    '\u{1FBD}', '\u{1FBF}', '\u{2018}', '\u{00B4}', '\u{0384}',
];

/// Correctly accented monosyllables. Does not contain pronouns.
pub const MONOSYLLABLE_ACCENTED: [&str; 12] =
    with_capitalized!(["ή", "πού", "πώς", "είς", "έν", "έξ"]);

/// Correctly accented monosyllables, including (accented versions of) pronouns.
pub const MONOSYLLABLE_ACCENTED_WITH_PRONOUNS: [&str; 28] = conc!(
    MONOSYLLABLE_ACCENTED,
    with_capitalized!(["μού", "μάς", "τού", "τής", "τούς", "τών", "σού", "σάς"])
);

/// Words with multiple accepted pronunciations.
///
/// Only contains words with accent not on the last syllable,
/// and with (possible) synizesis at the last syllable.
//
// This was automatically generated by scripts/synizesis/build.py.
// Do not edit manually.
#[rustfmt::skip]
pub const MULTIPLE_PRONUNCIATION: [&str; 76] = [
    "άδεια", "Άδεια", 
    "άδειας", "Άδειας", 
    "άδειες", "Άδειες", 
    "ακρίβεια", "Ακρίβεια", 
    "ακρίβειας", "Ακρίβειας", 
    "ακρίβειες", "Ακρίβειες", 
    "άγιος", "Άγιος", 
    "άγιου", "Άγιου", 
    "άγιο", "Άγιο", 
    "άγιε", "Άγιε", 
    "άγιοι", "Άγιοι", 
    "άγιων", "Άγιων", 
    "άγιους", "Άγιους", 
    "άγια", "Άγια", 
    "άγιας", "Άγιας", 
    "άγιες", "Άγιες", 
    "έννοια", "Έννοια", 
    "έννοιας", "Έννοιας", 
    "έννοιες", "Έννοιες", 
    "ήπια", "Ήπια", 
    "ήπιε", "Ήπιε", 
    "ήπιες", "Ήπιες", 
    "ίδιος", "Ίδιος", 
    "ίδιο", "Ίδιο", 
    "ίδιε", "Ίδιε", 
    "ίδιοι", "Ίδιοι", 
    "ίδια", "Ίδια", 
    "ήλιου", "Ήλιου", 
    "ήλιο", "Ήλιο", 
    "μύριοι", "Μύριοι", 
    "μύριων", "Μύριων", 
    "μύριους", "Μύριους", 
    "μύριες", "Μύριες", 
    "μύρια", "Μύρια", 
    "αρχοντολόγια", "Αρχοντολόγια", 
    "πλάγια", "Πλάγια", 
    "φυλάκια", "Φυλάκια", 
    "ουράνια", "Ουράνια", 
];
