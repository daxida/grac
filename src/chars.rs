use unicode_normalization::char::decompose_canonical;
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

// Doing this instead of ranges allows for const, but does not account for char gaps.
const fn is_modern_lower_greek(ch: char) -> bool {
    ch >= '\u{03B1}' && ch <= '\u{03C9}'
}

// Faster than an .nfd() call for a single char.
pub fn base(ch: char) -> char {
    let mut base_char = None;
    decompose_canonical(ch, |c| {
        base_char.get_or_insert(c);
    });
    base_char.unwrap_or(ch)
}

pub fn base_lower(ch: char) -> char {
    if is_modern_lower_greek(ch) {
        // Cheap function
        ch
    } else {
        // Expensive function
        base(ch).to_lowercase().next().unwrap_or(ch)
    }
}

fn extract_diacritic(diacritic: char) -> impl Fn(char) -> Option<char> {
    move |ch: char| ch.nfd().find(|&c| diacritic == c)
}

pub fn diaeresis(ch: char) -> Option<char> {
    let diaeresis = extract_diacritic(DIAERESIS);
    diaeresis(ch)
}
