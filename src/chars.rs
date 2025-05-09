//! Char utilities.
//!
//! Unicode Greek Ranges:
//! - [Greek and Coptic](https://www.unicode.org/charts/PDF/U0370.pdf)
//! - [Greek Extended](https://www.unicode.org/charts/PDF/U1F00.pdf)

// Match statements usually try to benefit from some fast path.
#![allow(clippy::match_same_arms)]

use unicode_normalization::char::decompose_canonical;

/// Check if a character is in the _Greek and Coptic_ range.
///
/// Includes the following non alphabetic chars:
///
/// 0x375: ͵ - GREEK LOWER NUMERAL SIGN
/// 0x37e: ; - GREEK QUESTION MARK
/// 0x384: ΄ - GREEK TONOS
/// 0x385: ΅ - GREEK DIALYTIKA TONOS
/// 0x387: · - GREEK ANO TELEIA
/// 0x3f6: ϶ - GREEK REVERSED LUNATE EPSILON SYMBOL
pub const fn is_greek_and_coptic_char(ch: char) -> bool {
    ch >= '\u{0370}' && ch <= '\u{03FF}'
}

/// Check if a character is in the _Greek Extended_ range.
///
/// Includes the following non alphabetic chars:
///
/// 0x1fbd: ᾽ - GREEK KORONIS
/// 0x1fbf: ᾿ - GREEK PSILI
/// 0x1fc0: ῀ - GREEK PERISPOMENI
/// 0x1fc1: ῁ - GREEK DIALYTIKA AND PERISPOMENI
/// 0x1fcd: ῍ - GREEK PSILI AND VARIA
/// 0x1fce: ῎ - GREEK PSILI AND OXIA
/// 0x1fcf: ῏ - GREEK PSILI AND PERISPOMENI
/// 0x1fdd: ῝ - GREEK DASIA AND VARIA
/// 0x1fde: ῞ - GREEK DASIA AND OXIA
/// 0x1fdf: ῟ - GREEK DASIA AND PERISPOMENI
/// 0x1fed: ῭ - GREEK DIALYTIKA AND VARIA
/// 0x1fee: ΅ - GREEK DIALYTIKA AND OXIA
/// 0x1fef: ` - GREEK VARIA
/// 0x1ffd: ´ - GREEK OXIA
/// 0x1ffe: ῾ - GREEK DASIA
pub const fn is_greek_extended_char(ch: char) -> bool {
    ch >= '\u{1F00}' && ch <= '\u{1FFF}'
}

/// Check if a character is in the _Greek and Coptic_ or the _Greek Extended_ range.
///
/// Includes non alphabetic characters.
///
/// For a version excluding non alphabetic characters, use [`is_greek_letter`].
pub const fn is_greek_char(ch: char) -> bool {
    is_greek_and_coptic_char(ch) || is_greek_extended_char(ch)
}

/// Check if a character is in the _Greek and Coptic_ or the _Greek Extended_ range.
///
/// Excludes non alphabetic characters.
///
/// For a version including non alphabetic characters, use [`is_greek_char`].
pub const fn is_greek_letter(ch: char) -> bool {
    match ch {
        '\u{375}' | '\u{37E}' | '\u{384}' | '\u{385}' | '\u{387}' | '\u{3F6}' | '\u{1FBD}'
        | '\u{1FBF}' | '\u{1FC0}' | '\u{1FC1}' | '\u{1FCD}' | '\u{1FCE}' | '\u{1FCF}'
        | '\u{1FDD}' | '\u{1FDE}' | '\u{1FDF}' | '\u{1FED}' | '\u{1FEE}' | '\u{1FEF}'
        | '\u{1FFD}' | '\u{1FFE}' => false,
        _ => is_greek_char(ch),
    }
}

/// Check if a word is Greek.
///
/// Note that from the three common characters that represent apostrophe:
/// * U+0027 ' APOSTROPHE
/// * U+2019 ’ RIGHT SINGLE QUOTATION MARK
/// * U+02BC ʼ MODIFIER LETTER APOSTROPHE
///
/// the last one is the only considered alphabetic, and since it can appear
/// as a possible (probably wrong) variant, it makes sense to include it here.
pub fn is_greek_word(s: &str) -> bool {
    // The order is important: is_greek_char is cheaper.
    s.chars()
        .all(|ch| is_greek_char(ch) || ch == '\u{02BC}' || !ch.is_alphabetic())
}

/// Check if the word ends with a diphthong.
///
/// Return true even when there are trailing consonants: Κάιν.
///
/// # Examples
///
/// ```
/// use grac::ends_with_diphthong;
///
/// assert_eq!(ends_with_diphthong("Κάιν"), true);
/// assert_eq!(ends_with_diphthong("πλάι"), true);
/// assert_eq!(ends_with_diphthong("γάιδαρος"), false);
/// ```
pub fn ends_with_diphthong(s: &str) -> bool {
    const DIPHTHONGS: [&str; 12] = [
        "όι", "Όι", "έι", "Έι", "άι", "Άι", "όυ", "Όυ", "έυ", "Έυ", "άυ", "Άυ",
    ];

    let vowels = extract_vowels(s);
    DIPHTHONGS.iter().any(|&e| vowels.ends_with(e))
}

/// Extract vowels from an assumed well formed lowercase syllable.
fn extract_vowels(s: &str) -> String {
    const CONSONANTS: [char; 35] = [
        // Lowercase
        'β', 'γ', 'δ', 'ζ', 'θ', 'κ', 'λ', 'μ', 'ν', 'ξ', 'π', 'ρ', 'σ', 'ς', 'τ', 'φ', 'χ', 'ψ',
        // Uppercase
        'Β', 'Γ', 'Δ', 'Ζ', 'Θ', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Π', 'Ρ', 'Σ', 'Τ', 'Φ', 'Χ', 'Ψ',
    ];
    s.chars().filter(|ch| !CONSONANTS.contains(ch)).collect()
}

/// Return the normalized character.
///
/// Oracle implementation for testing.
fn __base(ch: char) -> char {
    let mut base_char = None;
    decompose_canonical(ch, |c| {
        base_char.get_or_insert(c);
    });
    base_char.unwrap_or(ch)
}

/// Return the normalized lower character.
///
/// Oracle implementation for testing.
///
/// This should be equal to [`base_lower`](self::base_lower)
fn __base_lower(ch: char) -> char {
    __base(ch).to_lowercase().next().unwrap_or(ch)
}

/// Return the normalized lower character.
pub const fn base_lower(ch: char) -> char {
    match ch {
        // Greek and Coptic
        '\u{0370}'..='\u{03FF}' => base_lower_gc(ch),
        // Greek extended
        '\u{1F00}'..='\u{1FFF}' => base_lower_ge(ch),
        _ => ch,
    }
}

/// Normalize and cast to lowercase the _Greek and Coptic_ range.
///
/// Does NOT normalize:  ';' | '·' | 'Ϊ' | 'Ϋ' | 'ϓ' | 'ϔ'
const fn base_lower_gc(ch: char) -> char {
    match ch {
        // Lowercase unaccented (α > ω)
        // We want to check these first since they are most frequent.
        '\u{03B1}'..='\u{03C9}' => ch,

        // Lowercase accented vowels
        'ά' => 'α',
        'έ' => 'ε',
        'ί' => 'ι',
        'ή' => 'η',
        'ό' => 'ο',
        'ύ' => 'υ',
        'ώ' => 'ω',

        // Uppercase unaccented
        'Α' => 'α',
        'Β' => 'β',
        'Γ' => 'γ',
        'Δ' => 'δ',
        'Ε' => 'ε',
        'Ζ' => 'ζ',
        'Η' => 'η',
        'Θ' => 'θ',
        'Ι' => 'ι',
        'Κ' => 'κ',
        'Λ' => 'λ',
        'Μ' => 'μ',
        'Ν' => 'ν',
        'Ξ' => 'ξ',
        'Ο' => 'ο',
        'Π' => 'π',
        'Ρ' => 'ρ',
        'Σ' => 'σ',
        'Τ' => 'τ',
        'Υ' => 'υ',
        'Φ' => 'φ',
        'Χ' => 'χ',
        'Ψ' => 'ψ',
        'Ω' => 'ω',

        // Uppercase accented vowels
        'Ά' => 'α',
        'Έ' => 'ε',
        'Ί' => 'ι',
        'Ή' => 'η',
        'Ό' => 'ο',
        'Ύ' => 'υ',
        'Ώ' => 'ω',

        // Diereses && punctuation
        'ϊ' | 'ΐ' => 'ι',
        'ϋ' | 'ΰ' => 'υ',
        'ʹ' => 'ʹ',
        '΅' => '¨',
        _ => ch,
    }
}

/// Normalize and cast to lowercase the _Greek Extended_ range.
const fn base_lower_ge(ch: char) -> char {
    match ch {
        'ἀ' | 'ἁ' | 'ἂ' | 'ἃ' | 'ἄ' | 'ἅ' | 'ἆ' | 'ἇ' | 'Ἀ' | 'Ἁ' | 'Ἂ' | 'Ἃ' | 'Ἄ' | 'Ἅ' | 'Ἆ'
        | 'Ἇ' | 'ὰ' | 'ά' | 'ᾀ' | 'ᾁ' | 'ᾂ' | 'ᾃ' | 'ᾄ' | 'ᾅ' | 'ᾆ' | 'ᾇ' | 'ᾈ' | 'ᾉ' | 'ᾊ'
        | 'ᾋ' | 'ᾌ' | 'ᾍ' | 'ᾎ' | 'ᾏ' | 'ᾰ' | 'ᾱ' | 'ᾲ' | 'ᾳ' | 'ᾴ' | 'ᾶ' | 'ᾷ' | 'Ᾰ' | 'Ᾱ'
        | 'Ὰ' | 'Ά' | 'ᾼ' => 'α',
        'ἐ' | 'ἑ' | 'ἒ' | 'ἓ' | 'ἔ' | 'ἕ' | 'Ἐ' | 'Ἑ' | 'Ἒ' | 'Ἓ' | 'Ἔ' | 'Ἕ' | 'ὲ' | 'έ' | 'Ὲ'
        | 'Έ' => 'ε',
        'ἠ' | 'ἡ' | 'ἢ' | 'ἣ' | 'ἤ' | 'ἥ' | 'ἦ' | 'ἧ' | 'Ἠ' | 'Ἡ' | 'Ἢ' | 'Ἣ' | 'Ἤ' | 'Ἥ' | 'Ἦ'
        | 'Ἧ' | 'ὴ' | 'ή' | 'ᾐ' | 'ᾑ' | 'ᾒ' | 'ᾓ' | 'ᾔ' | 'ᾕ' | 'ᾖ' | 'ᾗ' | 'ᾘ' | 'ᾙ' | 'ᾚ'
        | 'ᾛ' | 'ᾜ' | 'ᾝ' | 'ᾞ' | 'ᾟ' | 'ῂ' | 'ῃ' | 'ῄ' | 'ῆ' | 'ῇ' | 'Ὴ' | 'Ή' | 'ῌ' => {
            'η'
        }
        'ἰ' | 'ἱ' | 'ἲ' | 'ἳ' | 'ἴ' | 'ἵ' | 'ἶ' | 'ἷ' | 'Ἰ' | 'Ἱ' | 'Ἲ' | 'Ἳ' | 'Ἴ' | 'Ἵ' | 'Ἶ'
        | 'Ἷ' | 'ὶ' | 'ί' | 'ι' | 'ῐ' | 'ῑ' | 'ῒ' | 'ΐ' | 'ῖ' | 'ῗ' | 'Ῐ' | 'Ῑ' | 'Ὶ' | 'Ί' => {
            'ι'
        }
        'ὀ' | 'ὁ' | 'ὂ' | 'ὃ' | 'ὄ' | 'ὅ' | 'Ὀ' | 'Ὁ' | 'Ὂ' | 'Ὃ' | 'Ὄ' | 'Ὅ' | 'ὸ' | 'ό' | 'Ὸ'
        | 'Ό' => 'ο',
        'ῤ' | 'ῥ' | 'Ῥ' => 'ρ',
        'ὐ' | 'ὑ' | 'ὒ' | 'ὓ' | 'ὔ' | 'ὕ' | 'ὖ' | 'ὗ' | 'Ὑ' | 'Ὓ' | 'Ὕ' | 'Ὗ' | 'ὺ' | 'ύ' | 'ῠ'
        | 'ῡ' | 'ῢ' | 'ΰ' | 'ῦ' | 'ῧ' | 'Ῠ' | 'Ῡ' | 'Ὺ' | 'Ύ' => 'υ',
        'ὠ' | 'ὡ' | 'ὢ' | 'ὣ' | 'ὤ' | 'ὥ' | 'ὦ' | 'ὧ' | 'Ὠ' | 'Ὡ' | 'Ὢ' | 'Ὣ' | 'Ὤ' | 'Ὥ' | 'Ὦ'
        | 'Ὧ' | 'ὼ' | 'ώ' | 'ᾠ' | 'ᾡ' | 'ᾢ' | 'ᾣ' | 'ᾤ' | 'ᾥ' | 'ᾦ' | 'ᾧ' | 'ᾨ' | 'ᾩ' | 'ᾪ'
        | 'ᾫ' | 'ᾬ' | 'ᾭ' | 'ᾮ' | 'ᾯ' | 'ῲ' | 'ῳ' | 'ῴ' | 'ῶ' | 'ῷ' | 'Ὼ' | 'Ώ' | 'ῼ' => {
            'ω'
        }
        '`' => '`',
        '῁' | '῭' | '΅' => '¨',
        '´' => '´',
        '῍' | '῎' | '῏' => '᾿',
        '῝' | '῞' | '῟' => '῾',
        _ => ch,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dbg_test(ch: char, received: char, expected: char) {
        if received != expected {
            println!(
                "[DBG] Char: {}, Bytes: {:?}, received: {} ({:?}), expected: {} ({:?})",
                ch,
                ch.encode_utf8(&mut [0; 4]).as_bytes(),
                received,
                received.encode_utf8(&mut [0; 4]).as_bytes(),
                expected,
                expected.encode_utf8(&mut [0; 4]).as_bytes(),
            );
        }
    }

    #[test]
    fn test_base_lower_greek_coptic() {
        for ch in '\u{0370}'..='\u{03FF}' {
            match ch {
                ';' | '·' | 'Ϊ' | 'Ϋ' | 'ϓ' | 'ϔ' | 'Ͱ' | 'Ͳ' | 'ʹ' | 'Ͷ' | 'Ϳ' | 'Ϗ' | 'Ϙ'
                | 'Ϝ' | 'Ϛ' | 'Ϟ' | 'Ϡ' | 'Ϣ' | 'Ϥ' | 'Ϧ' | 'Ϩ' | 'Ϫ' | 'Ϭ' | 'Ϯ' | 'ϴ' | 'Ϸ'
                | 'Ϲ' | 'Ϻ' | 'Ͻ' | 'Ͼ' | 'Ͽ' => continue,
                _ => (),
            }
            let expected = __base_lower(ch);
            let received = base_lower_gc(ch);
            dbg_test(ch, received, expected);
            assert_eq!(received, expected);
        }
    }

    #[test]
    fn test_base_lower_greek_extended() {
        for ch in '\u{1F00}'..='\u{1FFF}' {
            let expected = __base_lower(ch);
            let received = base_lower_ge(ch);
            dbg_test(ch, received, expected);
            assert_eq!(received, expected);
        }
    }

    #[test]
    fn test_is_greek_word() {
        let greek_words = [
            "καλημέρα",
            // U+0027 ' APOSTROPHE
            "ὑπ\u{0027}",
            // U+2019 ’ RIGHT SINGLE QUOTATION MARK
            "ὑπ\u{2019}",
            // U+02BC ʼ MODIFIER LETTER APOSTROPHE
            // Note that this one is alphabetic!
            "ὑπ\u{02BC}",
        ];
        for word in greek_words {
            assert!(is_greek_word(word), "Expected {word} to be a greek word.");
        }
    }
}
