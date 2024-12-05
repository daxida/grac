use unicode_normalization::UnicodeNormalization;

pub struct Accent;

impl Accent {
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
}

pub struct Breathing;

impl Breathing {
    // [  ̓ ] U+0313: ψιλή (psili)
    pub const SMOOTH: char = '\u{0313}';
    // [  ̔ ] U+0314: δασεία (dasia)
    pub const ROUGH: char = '\u{0314}';
}

fn has_diacritic(ch: char, diacritic: char) -> bool {
    ch.nfd().find(|&c| c == diacritic).is_some()
}

pub fn has_diaeresis(ch: char) -> bool {
    has_diacritic(ch, Accent::DIAERESIS)
}

fn _remove_diacritics(text: &str, diacritics: &[char]) -> String {
    text.nfd()
        .filter(|ch| !diacritics.contains(ch))
        .collect::<String>()
        .nfc()
        .to_string()
}

/// Remove all diacritics from a string.
///
/// ```
/// use grac::remove_diacritics;
///
/// let homer = "τὴν δ᾽ ἐγὼ οὐ λύσω: πρίν μιν καὶ γῆρας ἔπεισιν\n
///              ἡμετέρῳ ἐνὶ οἴκῳ ἐν Ἄργεϊ τηλόθι πάτρης";
/// assert_eq!(remove_diacritics(homer),
///             "την δ᾽ εγω ου λυσω: πριν μιν και γηρας επεισιν\n
///              ημετερω ενι οικω εν Αργει τηλοθι πατρης");
/// ```
pub fn remove_diacritics(text: &str) -> String {
    _remove_diacritics(
        text,
        &[
            Accent::CIRCUMFLEX,
            Accent::ACUTE,
            Accent::GRAVE,
            Accent::IOTA_SUBSCRIPT,
            Accent::DIAERESIS,
            Breathing::SMOOTH,
            Breathing::ROUGH,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_diacritics() {
        assert_eq!(remove_diacritics("λόγος ὁράω όι"), "λογος οραω οι");
    }
}
