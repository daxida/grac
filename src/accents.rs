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

// I've experienced issues with alternative characters
// for spaces, namely U+2009. Splitting by lines does only
// partially solve the issue (and it makes it slower...).
pub fn to_mono(text: &str) -> String {
    text.lines()
        .map(|line| {
            line.split_inclusive([' ', '-'])
                .map(to_mono_word)
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// HACK: Not a big fan
fn split_word_punctuation(word: &str) -> (&str, &str, &str) {
    let mut start = 0;
    let mut end = word.len();

    for (i, c) in word.char_indices() {
        if c.is_alphanumeric() {
            start = i;
            break;
        }
    }

    for (i, c) in word.char_indices().rev() {
        if c.is_alphanumeric() {
            end = i + c.len_utf8();
            break;
        }
    }

    let left_punct = &word[..start];
    let core_word = &word[start..end];
    let right_punct = &word[end..];

    (left_punct, core_word, right_punct)
}

// FIX: Does not belong here
const ACUTE_VOWELS_LOWER: [char; 7] = ['ά', 'έ', 'ή', 'ί', 'ό', 'ύ', 'ώ'];

/// True if it contains any lowercase accented letter.
/// Assumes a normalized string as input.
fn has_acute(s: &str) -> bool {
    s.chars().any(|c| ACUTE_VOWELS_LOWER.contains(&c))
}

// FIX: Does not belong here
const CONSONANTS: [char; 35] = [
    // Lowercase
    'β', 'γ', 'δ', 'ζ', 'θ', 'κ', 'λ', 'μ', 'ν', 'ξ', 'π', 'ρ', 'σ', 'ς', 'τ', 'φ', 'χ', 'ψ', 'Β',
    // Uppercase
    'Γ', 'Δ', 'Ζ', 'Θ', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Π', 'Ρ', 'Σ', 'Τ', 'Φ', 'Χ', 'Ψ',
];

/// Extract vowels from an assumed well formed lowercase syllable.
fn syllable_vowels(syl: &str) -> String {
    syl.chars().filter(|ch| !CONSONANTS.contains(ch)).collect()
}

/// Remove last acute accent (lowercase) if any.
fn remove_last_acute(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();

    if let Some(pos) = chars.iter().rposition(|c| ACUTE_VOWELS_LOWER.contains(c)) {
        chars[pos] = match chars[pos] {
            'ά' => 'α',
            'έ' => 'ε',
            'ή' => 'η',
            'ί' => 'ι',
            'ό' => 'ο',
            'ύ' => 'υ',
            'ώ' => 'ω',
            _ => chars[pos],
        };
    }

    chars.into_iter().collect()
}

// These are monosyllables from which we do not want to remove the accent
const MONOSYL_DO_NOT_REMOVE_ACCENT: [&str; 12] = [
    "ή", "Ή", "πού", "Πού", "πώς", "Πώς", "είς", "Είς", "έν", "Έν", "έξ", "Έξ",
];

// TODO: Decide on synizisi
//
// These are actually monosyllables, but not by our current logic.
// By putting them here they are return WITHOUT accent at the end.
const MONOSYL_REMOVE_ACCENT: [&str; 36] = [
    "πιό",
    "Πιό",
    "πιά",
    "Πιά",
    "πιώ",
    "Πιώ",
    "πίη",
    "Πίη",
    "μιά",
    "Μιά",
    "μιάς",
    "Μιάς",
    "γιά",
    "Γιά",
    "γειά",
    "Γειά",
    // πιω
    "πιώ",
    "Πιώ",
    "πίεις",
    "Πίεις",
    "πίη",
    "Πίη",
    "πιή",
    "Πιή",
    "πίει",
    "Πίει",
    "πιεί",
    "Πιεί",
    "πίης",
    "Πίης",
    "πιής",
    "Πιής",
    "πιούν",
    "Πιούν",
    "πιές",
    "Πιές",
];

// FIX: This does not belong here
const ACCENTED_DIPHTHONGS: [&str; 6] = ["όι", "Όι", "έι", "Έι", "άι", "Άι"];

#[allow(unused_variables)]
fn log(label: &str, value: impl std::fmt::Debug) {
    // println!("{:<30}: {:?}", label, value);
}

fn to_mono_word(word: &str) -> String {
    // For debug: ignore empty words
    if word.is_empty() {
        return String::new();
    }

    // Strip punct
    let (left_punct, word, right_punct) = split_word_punctuation(word);

    log("Input word", word);
    let bstring = word
        .as_bytes()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join(" ");
    log("Input bytes", bstring);

    // TODO: special cases
    let ret = match word {
        "ποὺ" => Some("που"),
        "πὼς" => Some("πως"),
        _ => None,
    };
    if let Some(s) = ret {
        return format!("{}{}{}", left_punct, s, right_punct);
    }

    let mut out = remove_non_accent_diacritics_without_dieresis(word);
    log("Non-accent diacritics removed", &out);

    out = out.nfd().to_string();
    log("Normalized to NFD", &out);

    out = grave_circumflex_to_acute(&out);
    log("Grave/circumflex to acute", &out);

    out = out.nfc().to_string();
    log("Re-normalized to NFC", &out);

    let syllables = syllabify_el(&out);
    log("Syllabified word", &syllables);

    const ABBREVIATION_MARKS: &str = "᾽᾿'";
    let fst_rpunct = right_punct.chars().next();

    out = match syllables.as_slice() {
        [syl] => {
            // Here: syl == out
            // consider adding similar graphs: '
            if !MONOSYL_DO_NOT_REMOVE_ACCENT.contains(syl)
                // The word should not end in an abbreviation mark: έτσ' είναι
                && ABBREVIATION_MARKS.chars().all(|ch| fst_rpunct != Some(ch))
            {
                // Do not change σόι, Κάιν
                let syl_vows = syllable_vowels(syl);
                if ACCENTED_DIPHTHONGS.iter().any(|&e| syl_vows.ends_with(e)) {
                    log("Monosyllable ending in diphthong", "Keeps accents");
                    out
                } else {
                    log("Monosyllable no accent", "Removing accents");
                    remove_accents(&out)
                }
            } else {
                log("Word keeps accents", &out);
                out
            }
        }
        [.., syl1, syl2] => {
            if MONOSYL_REMOVE_ACCENT.contains(&out.as_str()) {
                log("Word in NOT_ACCENTED list", "Removing accents");
                remove_accents(&out)
            } else if has_acute(syl1) && has_acute(syl2) {
                log("Two acute accents in two syllables", "Removing last acute");
                remove_last_acute(&out)
            } else {
                log("Word keeps accents", &out);
                out
            }
        }
        _ => out,
    };

    // We do this quite late to deal with Κέϋνς -> two syllables
    out = remove_superfluous_diaereses(&out);
    log("Removed superfluous diaereses", &out);

    log("Final transformed word", &out);
    let bstring = out
        .as_bytes()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join(" ");
    log("Input bytes", bstring);
    log("======================", "");

    format!("{}{}{}", left_punct, out, right_punct)
}

#[cfg(test)]
mod tests {
    use crate::accents::to_mono;

    use super::*;

    macro_rules! mktest_mono {
        ($group_name:ident, $([$input:expr, $expected:expr]),* $(,)?) => {
            #[test]
            fn $group_name() {
                let test_cases = vec![
                    $(
                        ($input, $expected),
                    )*
                ];

                for (input, expected) in test_cases {
                    let result = to_mono(input);
                    assert_eq!(result, expected);
                }
            }
        };
    }

    mktest_mono!(
        mono_special_cases,
        ["ποὺ", "που"],
        ["Ἐκεῖνον ποὺ μοῦ", "Εκείνον που μου"],
    );

    mktest_mono!(
        mono_hard_to_fix,
        // ["χλιός", "χλιος"],
        // ["Δαυίδ", "Δαυίδ"],
        ["δύο-τρεῖς", "δύο-τρεις"],
        ["λογιῶν-τῶν-λογιῶν", "λογιών-των-λογιών"],
        ["Ἅμα πιῇς τσάι", "Άμα πιης τσάι"],
    );

    mktest_mono!(
        mono_one_syl,
        ["πιό πιά πείς", "πιο πια πεις"],
        ["Ἂς πιῶ", "Ας πιω"],
        ["ὧν δι᾽ ὧν όν Τρῶν γρῦ γρί", "ων δι᾽ ων ον Τρων γρυ γρι"],
        ["τί θὲς;", "τι θες;"],
        ["ζείς", "ζεις"],
        ["Τὴν", "Την"],
        ["Ἔν τινι", "Έν τινι"],
        ["μιᾶς", "μιας"],
    );

    mktest_mono!(
        mono_diaeresis,
        ["γάϊδουρος", "γάιδουρος"],
        ["μπέϊκον", "μπέικον"],
        ["ἄϋλος", "άυλος"],
        ["φαΐ", "φαΐ"],
        ["ἀρχαϊκάς", "αρχαϊκάς"],
    );

    mktest_mono!(
        mono_diphthong_in_monosyllable,
        ["σόι", "σόι"],
        ["Γκρέι", "Γκρέι"],
        ["μάι", "μάι"],
        ["Κάιν", "Κάιν"],
        ["Μπόις", "Μπόις"],
    );

    // Happens in words that preceed stress-less words: τι, ποτε...
    mktest_mono!(
        mono_double_accents,
        // νὰ εἶναί τις Βενετός...
        // τὸν οἶκόν του μηδὲ στιγμὴν ἀναπαύσεως...
        // βαστάζοντες πρᾶγμά τι μακρὸν...
        ["εἶναί", "είναι"],
        ["οἶκόν", "οίκον"],
        ["πρᾶγμά", "πράγμα"],
        ["οἱ ὡραῖοί σου ὀδόντες,", "οι ωραίοι σου οδόντες,"],
        ["σφαγεῖόν", "σφαγείον"],
    );

    mktest_mono!(
        mono_double_accents_bis,
        ["ἐλαττώματά του", "ελαττώματά του"],
        ["τὸν σύζυγόν της", "τον σύζυγόν της"],
    );

    mktest_mono!(
        mono_punct,
        // Do not treat abbreviation as monosyllables
        ["Ὅλ᾽", "Όλ᾽"],
        ["ὅλ᾿ αἱ", "όλ᾿ αι"],
        ["ἔτσ᾿ εἶναι", "έτσ᾿ είναι"],
        ["έτσ' είναι", "έτσ' είναι"],
        ["οὔτ᾿ ἐνθύμηση", "ούτ᾿ ενθύμηση"],
        ["Τί κάν᾽ ἡ λεχώνα;", "Τι κάν᾽ η λεχώνα;"],
        // Starting punctuation
        ["«τὸ", "«το"],
        ["―Σπαργανίσαμε*", "―Σπαργανίσαμε*"],
        ["διηγῆται:\n\n―\u{2009}Τὴν", "διηγήται:\n\n―\u{2009}Την"],
        ["ἐμάλωσες·", "εμάλωσες·"],
        // Punctuation inside
        // ["Ναί…ναί…", "Ναι…ναι…"],
        // Others
        ["ὅ,τι", "ό,τι"],
        ["Ὅ,τι τοῦ τύχῃ", "Ό,τι του τύχη"],
        ["Ό,τ᾿ είναι", "Ό,τ᾿ είναι"],
        // We can not guess before elipsis: it may be either an actual
        // monosyllable or a truncated word.
        // We decide to go with the removal because it is more likely
        // that the word is not truncated.
        ["πρά…", "πρα…"], // This could be πράσινο...
        ["Ναί… ναί…", "Ναι… ναι…"],
    );

    mktest_mono!(
        mono_names,
        ["Τζὼν", "Τζων"],
        ["Μέυναρντ", "Μέυναρντ"],
        ["Κέϋνς", "Κέυνς"],
        [
            "Σὲρ Ἄρθουρ Ἰγνάτιος Κόναν Ντόϊλ",
            "Σερ Άρθουρ Ιγνάτιος Κόναν Ντόιλ"
        ],
    );

    mktest_mono!(
        mono_capital,
        ["ΩΙΔΗ ΑΙΘΟΥΣΑ ΗΙΩΝΑ", "ΩΙΔΗ ΑΙΘΟΥΣΑ ΗΙΩΝΑ"],
        ["ΕΛΛΑΔΑ", "ΕΛΛΑΔΑ"]
    );

    mktest_mono!(
        mono_papadiamantis,
        [
            "τῆς ἔδωκαν νὰ πίῃ διάφορα τελεσιουργὰ",
            "της έδωκαν να πιη διάφορα τελεσιουργά"
        ],
        ["κάμνει μία χόπ! καὶ βγαίνει", "κάμνει μία χοπ! και βγαίνει"],
        [
            "δὲν θὰ σ ἀφήσουνε νὰ μπεῖς ἐκεῖ",
            "δεν θα σ αφήσουνε να μπεις εκεί"
        ],
        ["ἀρχαϊκάς", "αρχαϊκάς"],
    );

    #[test]
    fn test_remove_diacritics() {
        assert_eq!(remove_accents("όι άι έι ύι όυ"), "οϊ αϊ εϊ υϊ οϋ");
        assert_eq!(remove_accents("λόγος ὁράω"), "λογος ὁραω");
    }
}
