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

/// Correctly accented monosyllables. Does not contain pronouns.
pub const MONOSYLLABLE_ACCENTED: [&str; 12] =
    with_capitalized!(["ή", "πού", "πώς", "είς", "έν", "έξ"]);

/// Correctly accented monosyllables, including (accented versions of) pronouns.
pub const MONOSYLLABLE_ACCENTED_WITH_PRONOUNS: [&str; 28] = conc!(
    MONOSYLLABLE_ACCENTED,
    with_capitalized!(["μού", "μάς", "τού", "τής", "τούς", "τών", "σού", "σάς"])
);

/// Greek words with two accepted syllabifications
///
// TODO: <https://el.wiktionary.org/wiki/Κατηγορία:Λέξεις_με_δύο_προφορές_(νέα_ελληνικά)>
pub const ALT_SYLLABIC: [&str; 34] = with_capitalized!(conc!(
    // έννοια
    expand!(["έννοι"], ["α", "ας", "ες"]),
    // πίνω / ήπιος
    expand!(["ήπι"], ["α", "ες"]),
    // ίδιος
    expand!(
        ["ίδι"],
        ["ος", "ου", "ο", "ε", "οι", "ων", "ους", "α", "ας", "ες"]
    ),
    // ήλιος / ήλιο
    expand!(["ήλι"], ["ου", "ο"])
));

#[cfg(test)]
mod tests {
    use super::*;

    fn is_alt_syllabic(word: &str) -> bool {
        ALT_SYLLABIC.contains(&word)
    }

    #[test]
    fn test_alt_syllabic() {
        assert!(is_alt_syllabic("ήλιο"));
        assert!(is_alt_syllabic("Ήλιο"));
        assert!(!is_alt_syllabic("ήλιος"));
    }
}
