use crate::syllabify::{is_vowel_el, syllabify_el};
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

/// Checks if the char or &str contains the specified diacritic.
///
/// # Examples
///
/// ```
/// # use grac::{Diacritic, has_diacritic};
///
/// assert_eq!(has_diacritic('α', Diacritic::GRAVE), false);
/// assert_eq!(has_diacritic('ϊ', Diacritic::DIAERESIS), true);
/// assert_eq!(has_diacritic("γάϊδουρος", Diacritic::ACUTE), true);
/// ```
pub fn has_diacritic<I>(word: impl UnicodeNormalization<I>, diacritic: char) -> bool
where
    I: Iterator<Item = char>,
{
    word.nfd().any(|c| c == diacritic)
}

pub fn has_diaeresis<I>(word: impl UnicodeNormalization<I>) -> bool
where
    I: Iterator<Item = char>,
{
    has_diacritic(word, Diacritic::DIAERESIS)
}

pub fn has_acute<I>(word: impl UnicodeNormalization<I>) -> bool
where
    I: Iterator<Item = char>,
{
    has_diacritic(word, Diacritic::ACUTE)
}

/// Remove specified diacritics from a string.
///
/// # Examples
///
/// ```
/// use grac::{remove_diacritics, Diacritic};
///
/// let diacritics = [Diacritic::GRAVE, Diacritic::SMOOTH];
/// let text = "ἄνθρωπος ἐστὶ";
/// let res  = "άνθρωπος εστι";
/// assert_eq!(remove_diacritics(text, &diacritics), res);
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

pub fn remove_acute(text: &str) -> String {
    remove_diacritics(text, &[Diacritic::ACUTE])
}

/// Remove diacritic to the specified syllable of a word.
///
/// # Examples
///
/// ```
/// use grac::{remove_diacritic_at, Diacritic};
///
/// assert_eq!(remove_diacritic_at("άνθρωπέ", 1, Diacritic::ACUTE), "άνθρωπε");
/// ```
pub fn remove_diacritic_at(word: &str, pos: usize, diacritic: char) -> String {
    let mut syllables: Vec<&str> = syllabify_el(word);

    if pos == 0 || pos > syllables.len() {
        word.to_string()
    } else {
        let idx = syllables.len() - pos;
        let replace_with = remove_diacritics(syllables[idx], &[diacritic]);
        syllables[idx] = replace_with.as_str();
        syllables.join("")
    }
}

/// Add an acute accent to the specified syllable of a word.
///
/// The syllable position starts at one and is counted from the end of the word.
///
/// # Examples
///
/// ```
/// use grac::add_acute_at;
///
/// assert_eq!(add_acute_at("ανθρωπος", 1), "ανθρωπός");
/// assert_eq!(add_acute_at("ανθρωπος", 2), "ανθρώπος");
/// assert_eq!(add_acute_at("ανθρωπος", 3), "άνθρωπος");
/// assert_eq!(add_acute_at("ανθρωπος", 4), "ανθρωπος");
///
/// // It may not yield the expected result based on the
/// // syllabification of the word.
/// assert_eq!(add_acute_at("σοι", 1), "σοί");
/// ```
pub fn add_acute_at(word: &str, pos: usize) -> String {
    add_diacritic_at(word, pos, Diacritic::ACUTE)
}

fn add_diacritic_at(word: &str, pos: usize, diacritic: char) -> String {
    let mut syllables: Vec<&str> = syllabify_el(word);

    if pos == 0 || pos > syllables.len() {
        word.to_string()
    } else {
        let idx = syllables.len() - pos;
        let replace_with = add_diacritic_at_syllable(syllables[idx], diacritic);
        syllables[idx] = replace_with.as_str();
        syllables.join("")
    }
}

/// Add diacritic to the first vowel from the end.
///
/// This is not ideal and could not yield the expected result.
fn add_diacritic_at_syllable(syllable: &str, diacritic: char) -> String {
    let mut chars: Vec<char> = syllable.chars().collect();
    if let Some(pos) = chars.iter().rposition(|ch| is_vowel_el(*ch)) {
        chars[pos] = add_diacritic_to_char(chars[pos], diacritic);
    }
    chars.into_iter().collect()
}

fn add_diacritic_to_char(ch: char, diacritic: char) -> char {
    format!("{ch}{diacritic}").nfc().next().unwrap_or(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_acute() {
        assert_eq!(add_diacritic_to_char('α', Diacritic::ACUTE), 'ά');
        assert_eq!(add_diacritic_to_char('Ω', Diacritic::GRAVE), 'Ὼ');
        assert_eq!(add_diacritic_to_char('ά', Diacritic::ACUTE), 'ά');
    }
}
