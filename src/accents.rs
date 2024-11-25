use unicode_normalization::UnicodeNormalization;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Breathing {
    Smooth, // "\u{0313}"
    Rough,  // "\u{0314}"
    Psili,  // Alias for Smooth
    Dasia,  // Alias for Rough
}

impl Breathing {
    pub const fn as_char(self) -> char {
        match self {
            Breathing::Smooth | Breathing::Psili => '\u{0313}',
            Breathing::Rough | Breathing::Dasia => '\u{0314}',
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accent {
    Acute,       // "\u{0301}"
    Grave,       // "\u{0300}"
    Circumflex,  // "\u{0342}"
    Oxia,        // Alias for Acute
    Varia,       // Alias for Grave
    Perispomeni, // Alias for Circumflex
}

impl Accent {
    pub const fn as_char(self) -> char {
        match self {
            Accent::Acute | Accent::Oxia => '\u{0301}',
            Accent::Grave | Accent::Varia => '\u{0300}',
            Accent::Circumflex | Accent::Perispomeni => '\u{0342}',
        }
    }
}

const DIAERESIS: char = '\u{0308}';

fn extract_diacritic(diacritic: char) -> impl Fn(char) -> Option<char> {
    move |ch: char| ch.nfd().find(|&c| diacritic == c)
}

pub fn diaeresis(ch: char) -> Option<char> {
    let diaeresis = extract_diacritic(DIAERESIS);
    diaeresis(ch)
}
