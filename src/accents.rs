use unicode_normalization::UnicodeNormalization;

use crate::chars::is_greek_char;
use crate::syllabify::syllabify_el;

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

fn replace_from_str_ary(text: &str, replacements: &[(&str, &str)]) -> String {
    let mut result = text.to_string();
    for &(from, to) in replacements {
        result = result.replace(from, to);
    }
    result
}

// fn replace_from_char_ary(text: &str, replacements: &[(char, char)]) -> String {
//     text.chars()
//         .map(|x| {
//             replacements
//                 .iter()
//                 .find(|&&(from, _)| from == x)
//                 .map_or(x, |&(_, to)| to)
//         })
//         .collect()
// }

fn remove_superfluous_diaereses(text: &str) -> String {
    const SUPERFLUOUS_DIAERESES: [(&str, &str); 6] = [
        ("άϊ", "άι"),
        ("άϋ", "άυ"),
        ("έϊ", "έι"),
        ("έϋ", "έυ"),
        ("όϊ", "όι"),
        ("ούϊ", "ούι"),
    ];
    replace_from_str_ary(text, &SUPERFLUOUS_DIAERESES)
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

/// Convert polytonic to monotonic Greek.
///
/// Leaves non greek words unchanged.
///
/// ```
/// use grac::*;
///
/// let input = "Ἑλλάς καὶ κόσμος.\r\n...ἄνθρωπος.";
/// let result = to_mono(input);
/// assert_eq!(result, "Ελλάς και κόσμος.\r\n...άνθρωπος.");
/// ```
pub fn to_mono(text: &str) -> String {
    text.split_inclusive(|c: char| c.is_whitespace() || c == '-')
        .map(to_mono_word)
        .collect()
}

fn split_word_punctuation(word: &str) -> (&str, &str, &str) {
    let start = word
        .char_indices()
        .find(|&(_, c)| c.is_alphabetic())
        .map(|(i, _)| i);

    let end = word
        .char_indices()
        .rev()
        .find(|&(_, c)| c.is_alphabetic())
        .map(|(i, c)| i + c.len_utf8());

    if let Some(start) = start {
        let end = end.unwrap();
        (&word[..start], &word[start..end], &word[end..])
    } else {
        // If the word has not a single alphabetic char...
        (word, "", "")
    }
}

// FIX: Does not belong here
const ACUTE_VOWELS_LOWER: [char; 7] = ['ά', 'έ', 'ή', 'ί', 'ό', 'ύ', 'ώ'];

/// True if it contains any lowercase accented letter.
/// Assumes a normalized string as input.
fn has_acute(s: &str) -> bool {
    s.chars().any(|c| ACUTE_VOWELS_LOWER.contains(&c))
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

fn ends_in_diphthong(s: &str) -> bool {
    ["όι", "Όι", "έι", "Έι", "άι", "Άι"]
        .iter()
        .any(|&e| s.ends_with(e))
}

fn is_greek_word(word: &str) -> bool {
    word.chars()
        .all(|ch| !ch.is_alphabetic() || is_greek_char(ch))
}

#[allow(unused_variables)]
fn log(label: &str, value: impl std::fmt::Debug) {
    // println!("{:<30}: {:?}", label, value);
}

fn to_mono_word(word: &str) -> String {
    // For debug: ignore empty words
    if word.is_empty() {
        return String::new();
    }

    // Do not remove accents if the word is not greek
    if !is_greek_word(word) {
        log("Not a greek word!", word);
        return word.to_string();
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

    // Remove ancient diacritics and convert grave and circumflex to acute
    let mut out = word
        .nfd()
        // Remove ancient diacritics
        .filter(|c| ![Accent::IOTA_SUBSCRIPT, Breathing::ROUGH, Breathing::SMOOTH].contains(c))
        // Grave and circumflex to acute
        .map(|c| match c {
            Accent::GRAVE | Accent::CIRCUMFLEX => Accent::ACUTE,
            _ => c,
        })
        .nfc()
        .collect::<String>();

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
                if ends_in_diphthong(&extract_vowels(syl)) {
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
    use super::*;

    #[test]
    fn test_remove_accents() {
        assert_eq!(remove_accents("λόγος ὁράω όι"), "λογος ὁραω οι");
    }

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
        mono_not_greek,
        ["1808·", "1808·"],
        [
            "Le Poète est semblable au prince des nuées",
            "Le Poète est semblable au prince des nuées"
        ],
        [
            "Qui hante la tempête et se rit de l'archer;",
            "Qui hante la tempête et se rit de l'archer;"
        ],
        [
            "Exilé sur le sol au milieu des huées,",
            "Exilé sur le sol au milieu des huées,"
        ],
        [
            "Ses ailes de géant l'empêchent de marcher.",
            "Ses ailes de géant l'empêchent de marcher."
        ],
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
}
