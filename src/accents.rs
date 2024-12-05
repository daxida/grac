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

fn extract_diacritic(ch: char, diacritic: char) -> Option<char> {
    ch.nfd().find(|&c| c == diacritic)
}

pub fn diaeresis(ch: char) -> Option<char> {
    extract_diacritic(ch, Accent::DIAERESIS)
}

fn remove_diacritics(text: &str, diacritics: &[char]) -> String {
    text.nfd()
        .filter(|ch| !diacritics.contains(ch))
        .collect::<String>()
        .nfc()
        .to_string()
}

pub fn remove_accents(text: &str) -> String {
    remove_diacritics(text, &[Accent::CIRCUMFLEX, Accent::ACUTE, Accent::GRAVE])
}
