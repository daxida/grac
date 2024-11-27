use unicode_normalization::UnicodeNormalization;

use crate::syllabify::syllabify_el;

/// - `ACUTE`          (U+0301):  ́         OXIA
/// - `GRAVE`          (U+0300):  ̀         VARIA
/// - `CIRCUMFLEX`     (U+0342):  ͂         PERISPOMENI
/// - `IOTA_SUBSCRIPT` (U+0345):  ͅ         YPOGEGRAMMENI
/// - `DIAERESIS`      (U+0308):  ̈         None
pub struct Accent;

impl Accent {
    pub const ACUTE: char = '\u{0301}';
    pub const GRAVE: char = '\u{0300}';
    pub const CIRCUMFLEX: char = '\u{0342}';
    pub const IOTA_SUBSCRIPT: char = '\u{0345}';
    pub const DIAERESIS: char = '\u{0308}';
    // Greek aliases
    pub const OXIA: char = Self::ACUTE;
    pub const VARIA: char = Self::GRAVE;
    pub const PERISPOMENI: char = Self::CIRCUMFLEX;
    pub const YPOGEGRAMMENI: char = Self::IOTA_SUBSCRIPT;
}

/// - `SMOOTH` (U+0313):  ̓         PSILI
/// - `ROUGH`  (U+0314):  ̔         DASIA
pub struct Breathing;

impl Breathing {
    pub const SMOOTH: char = '\u{0313}';
    pub const ROUGH: char = '\u{0314}';
    // Greek aliases
    pub const PSILI: char = Self::SMOOTH;
    pub const DASIA: char = Self::ROUGH;
}

const fn extract_diacritic(diacritic: char) -> impl Fn(char) -> Option<char> {
    move |ch: char| ch.nfd().find(|&c| diacritic == c)
}

// TODO: make this const
pub fn diaeresis(ch: char) -> Option<char> {
    let diaeresis = extract_diacritic(Accent::DIAERESIS);
    diaeresis(ch)
}

const ACUTE_TO_DIAERESIS: [(&str, &str); 5] = [
    ("όι", "οϊ"),
    ("άι", "αϊ"),
    ("έι", "εϊ"),
    ("ύι", "υϊ"),
    ("όυ", "οϋ"),
];

// use aho_corasick::AhoCorasick;
// const ACUTE_FR: [&str; 5] = ["όι", "άι", "έι", "ύι", "όυ"];
// const DIAERESIS_TO: [&str; 5] = ["οϊ", "αϊ", "εϊ", "υϊ", "οϋ"];

pub fn acute_to_diaeresis(text: &str) -> String {
    // let ac = AhoCorasick::new(ACUTE_FR).unwrap();
    // ac.replace_all(text, &DIAERESIS_TO)
    let mut ntext = text.to_string();
    for (from, to) in ACUTE_TO_DIAERESIS {
        ntext = ntext.replace(from, to)
    }
    ntext
}

pub fn diaeresis_to_acute(text: &str) -> String {
    let mut ntext = text.to_string();
    for (to, from) in ACUTE_TO_DIAERESIS {
        ntext = ntext.replace(from, to)
    }
    ntext
}

const SUPERFLUOUS_DIAERESES: [(&str, &str); 6] = [
    ("άϊ", "άι"),
    ("άϋ", "άυ"),
    ("έϊ", "έι"),
    ("έϋ", "έυ"),
    ("όϊ", "όι"),
    ("ούϊ", "ούι"),
];

pub fn remove_superfluous_diaereses(text: &str) -> String {
    let mut ntext = text.to_string();
    for (from, to) in SUPERFLUOUS_DIAERESES {
        ntext = ntext.replace(from, to)
    }
    ntext
}

/// Factory function for customizing functions that remove accents.
pub const fn remove_diacritics(
    diacritics: &[char],
    diaeresis: bool,
) -> impl Fn(&str) -> String + '_ {
    move |text: &str| -> String {
        let mut text = text.to_string();
        if diaeresis {
            // why?
            text = acute_to_diaeresis(&text)
        }
        text.nfd()
            .filter(|ch| !diacritics.contains(ch))
            .collect::<String>()
            .nfc()
            .to_string()
    }
}

/// ```
/// use grac::remove_accents;
///
/// let input = "λόγος ὁράω όι άι έι ύι όυ";
/// let result = remove_accents(input);
/// assert_eq!(result, "λογος ὁραω οϊ αϊ εϊ υϊ οϋ");
/// ```
// FIXME: How to make const? With a macro maybeh
pub fn remove_accents(text: &str) -> String {
    remove_diacritics(&[Accent::CIRCUMFLEX, Accent::ACUTE, Accent::GRAVE], true)(text)
}

pub fn remove_non_accent_diacritics_without_dieresis(text: &str) -> String {
    remove_diacritics(
        &[Accent::IOTA_SUBSCRIPT, Breathing::ROUGH, Breathing::SMOOTH],
        false,
    )(text)
}

// FIXME: be less fugly
pub fn grave_circumflex_to_acute(text: &str) -> String {
    let mut ntext = text.to_string();
    ntext = ntext.replace(Accent::GRAVE, &Accent::ACUTE.to_string());
    ntext = ntext.replace(Accent::CIRCUMFLEX, &Accent::ACUTE.to_string());
    ntext
}

pub fn to_mono(text: &str) -> String {
    text.split(' ')
        .map(to_mono_word)
        .collect::<Vec<_>>()
        .join(" ")
}

const MONOSYL_ACCENTED: [&str; 3] = ["ή", "πού", "πώς"];
// These are actually monosyllables, but not by our current logic
const NOT_MONOSYL_NOT_ACCENTED: [&str; 4] = ["πιό", "πιά", "πιώ", "πίη"];

fn log(label: &str, value: impl std::fmt::Debug) {
    // println!("{:<30}: {:?}", label, value);
}

fn to_mono_word(word: &str) -> String {
    // For debug: ignore empty words
    if word.is_empty() {
        return String::new();
    }
    log("Input word", word);

    // TODO: special cases

    let mut out = remove_non_accent_diacritics_without_dieresis(word);
    log("Non-accent diacritics removed", &out);

    // HERE: remove superfluous diaereses

    out = out.nfd().to_string();
    log("Normalized to NFD", &out);

    out = grave_circumflex_to_acute(&out);
    log("Grave/circumflex to acute", &out);

    out = out.nfc().to_string();
    log("Re-normalized to NFC", &out);

    let syllables = syllabify_el(&out);
    log("Syllabified word", &syllables);

    out = match syllables.len() {
        1 if !MONOSYL_ACCENTED.contains(&out.as_str()) => {
            log("Monosyllable no accent", "Removing accents");
            remove_accents(&out)
        }
        _ => {
            if NOT_MONOSYL_NOT_ACCENTED.contains(&out.as_str()) {
                log("Word in NOT_ACCENTED list", "Removing accents");
                remove_accents(&out)
            } else {
                log("Word keeps accents", &out);
                out
            }
        }
    };

    // We do this quite late to deal with Κέϋνς -> two syllables
    out = remove_superfluous_diaereses(&out);
    log("Removed superfluous diaereses", &out);

    out = diaeresis_to_acute(&out);
    log("Diaeresis to acute", &out);

    log("Final transformed word", &out);
    log("======================", "");
    out
}

#[cfg(test)]
mod tests {
    use crate::accents::to_mono;

    use super::*;

    macro_rules! mkmonotest {
        ($fn_name:ident, $poly:expr, $mono:expr) => {
            #[test]
            fn $fn_name() {
                let poly = $poly.trim();
                let mono = $mono.trim();
                let result = to_mono(poly);
                let received: Vec<&str> = result.lines().map(str::trim).collect();
                let expected: Vec<&str> = mono.lines().map(str::trim).collect();

                assert_eq!(received.len(), expected.len(), "Line count mismatch");
                for (r, e) in received.iter().zip(expected.iter()) {
                    assert_eq!(r, e, "Mismatch on line: expected '{}', got '{}'", e, r);
                }
            }
        };
    }

    mkmonotest!(
        mono_one_syl,
        r##"
            πιό πιά πείς
            Ἂς πιῶ
        "##,
        r##"
            πιο πια πεις
            Ας πιω
        "##
    );

    mkmonotest!(
        mono_one_syl_two,
        r##"
            ὧν δι᾽ ὧν όν Τρῶν γρῦ γρί
            τί θὲς;
        "##,
        r##"
            ων δι᾽ ων ον Τρων γρυ γρι
            τι θες;
        "##
    );

    mkmonotest!(
        mono_diaeresis,
        r##"
            γάϊδουρος μπέϊκον ἄϋλος
        "##,
        r##"
            γάιδουρος μπέικον άυλος
        "##
    );

    mkmonotest!(
        mono_diaeresis_two,
        r##"
            Τζὼν Μέυναρντ Κέϋνς
        "##,
        r##"
            Τζων Μέυναρντ Κέυνς
        "##
    );

    mkmonotest!(
        mono_two,
        r##"
            Σὲρ Ἄρθουρ Ἰγνάτιος Κόναν Ντόϊλ
            ζείς
            πεντάκις τῆς ἔδωκαν νὰ πίῃ διάφορα τελεσιουργὰ βότανα
            φαΐ
            κάμνει μία χόπ! καὶ βγαίνει
            δὲν θὰ σ ἀφήσουνε νὰ μπεῖς ἐκεῖ
            ΩΙΔΗ ΑΙΘΟΥΣΑ ΗΙΩΝΑ
        "##,
        r##"
            Σερ Άρθουρ Ιγνάτιος Κόναν Ντόιλ
            ζεις
            πεντάκις της έδωκαν να πιη διάφορα τελεσιουργά βότανα
            φαΐ
            κάμνει μία χοπ! και βγαίνει
            δεν θα σ αφήσουνε να μπεις εκεί
            ΩΙΔΗ ΑΙΘΟΥΣΑ ΗΙΩΝΑ
        "##
    );

    #[test]
    fn test_remove_diacritics() {
        assert_eq!(remove_accents("όι άι έι ύι όυ"), "οϊ αϊ εϊ υϊ οϋ");
        assert_eq!(remove_accents("λόγος ὁράω"), "λογος ὁραω");
    }
}
