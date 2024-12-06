use unicode_normalization::UnicodeNormalization;

pub struct Diacritic;

impl Diacritic {
    // [  ́ ] U+0301: οξεία (oxia)
    pub const ACUTE: char = '\u{0301}';
    // [  ̀ ] U+0300: βαρεία (varia)
    pub const GRAVE: char = '\u{0300}';
    // [  ͂ ] U+0342: περισπωμένη (perispomeni)
    pub const CIRCUMFLEX: char = '\u{0342}';
    // [  ͅ ] U+0345: υπογεγραμμένη (ypogegrammeni)
    pub const IOTA_SUBSCRIPT: char = '\u{0345}';
    // [  ̈ ] U+0308: διαλυτικά (diaeresis)
    pub const DIAERESIS: char = '\u{0308}';
    // [  ̓ ] U+0313: ψιλή (psili)
    pub const SMOOTH: char = '\u{0313}';
    // [  ̔ ] U+0314: δασεία (dasia)
    pub const ROUGH: char = '\u{0314}';
}

fn has_diacritic(ch: char, diacritic: char) -> bool {
    ch.nfd().any(|c| c == diacritic)
}

pub fn has_diaeresis(ch: char) -> bool {
    has_diacritic(ch, Diacritic::DIAERESIS)
}

/// Remove specified diacritics from a string.
///
/// # Examples
///
/// ```
/// use grac::{remove_diacritics, Diacritic};
///
/// let text = "ἄνθρωπος ἐστὶ";
/// assert_eq!(
///     remove_diacritics(text, &[Diacritic::GRAVE, Diacritic::SMOOTH]),
///     "άνθρωπος εστι"
/// );
/// ```
pub fn remove_diacritics(text: &str, diacritics: &[char]) -> String {
    text.nfd()
        .filter(|ch| !diacritics.contains(ch))
        .collect::<String>()
        .nfc()
        .to_string()
}

/// Remove all diacritics from a string.
///
/// # Examples
///
/// ```
/// use grac::remove_all_diacritics;
///
/// let homer = "τὴν δ᾽ ἐγὼ οὐ λύσω: πρίν μιν καὶ γῆρας ἔπεισιν\n
///              ἡμετέρῳ ἐνὶ οἴκῳ ἐν Ἄργεϊ τηλόθι πάτρης";
/// assert_eq!(remove_all_diacritics(homer),
///             "την δ᾽ εγω ου λυσω: πριν μιν και γηρας επεισιν\n
///              ημετερω ενι οικω εν Αργει τηλοθι πατρης");
/// ```
pub fn remove_all_diacritics(text: &str) -> String {
    remove_diacritics(
        text,
        &[
            Diacritic::CIRCUMFLEX,
            Diacritic::ACUTE,
            Diacritic::GRAVE,
            Diacritic::IOTA_SUBSCRIPT,
            Diacritic::DIAERESIS,
            Diacritic::SMOOTH,
            Diacritic::ROUGH,
        ],
    )
}

use crate::syllabify::is_vowel_el;
use crate::syllabify_el;

/// Add an acute accent to the specified syllable of a word.
///
/// The position is counted from the end of the word (in syllables). Starts at 1.
///
/// # Examples
///
/// ```
/// use grac::add_acute;
///
/// assert_eq!(add_acute("ανθρωπος", 1), "ανθρωπός");
/// assert_eq!(add_acute("ανθρωπος", 2), "ανθρώπος");
/// assert_eq!(add_acute("ανθρωπος", 3), "άνθρωπος");
/// assert_eq!(add_acute("ανθρωπος", 4), "ανθρωπος");
/// ```
pub fn add_acute(word: &str, pos: usize) -> String {
    let syllables = syllabify_el(word);

    if pos == 0 || pos > syllables.len() {
        return word.to_string();
    }

    // NOTE: use rposition?
    syllables
        .iter()
        .enumerate()
        .map(|(i, syllable)| {
            if i == syllables.len() - pos {
                add_acute_to_syllable(syllable)
            } else {
                syllable.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Add acute to the first vowel from the end.
/// NOTE: This is not ideal and could not yield the expected result.
fn add_acute_to_syllable(syllable: &str) -> String {
    let mut chars: Vec<char> = syllable.chars().collect();
    if let Some(pos) = chars.iter().rposition(|ch| is_vowel_el(*ch)) {
        chars[pos] = add_diacritic(chars[pos], Diacritic::ACUTE);
    }
    chars.into_iter().collect()
}

fn add_diacritic(ch: char, diacritic: char) -> char {
    format!("{ch}{diacritic}").nfc().next().unwrap_or(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_acute() {
        assert_eq!(add_diacritic('α', Diacritic::ACUTE), 'ά');
        assert_eq!(add_diacritic('Ω', Diacritic::GRAVE), 'Ὼ');
    }
}
