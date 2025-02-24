use unicode_normalization::UnicodeNormalization;

use crate::accents::Diacritic;
use crate::accents::{has_acute, remove_acute, remove_diacritic_at};
use crate::chars::{ends_with_diphthong, is_greek_word};
use crate::constants::{APOSTROPHES, MONOSYLLABLE_ACCENTED};
use crate::is_greek_letter;
use crate::syllabify::syllabify_el;

fn replace_from_str_ary(s: &str, replacements: &[(&str, &str)]) -> String {
    let mut result = s.to_string();
    for &(from, to) in replacements {
        result = result.replace(from, to);
    }
    result
}

fn remove_superfluous_diaereses(s: &str) -> String {
    const SUPERFLUOUS_DIAERESES: [(&str, &str); 7] = [
        ("άϊ", "άι"),
        ("άϋ", "άυ"),
        ("έϊ", "έι"),
        ("έϋ", "έυ"),
        ("όϊ", "όι"),
        ("όϋ", "όυ"),
        ("ούϊ", "ούι"),
    ];
    replace_from_str_ary(s, &SUPERFLUOUS_DIAERESES)
}

/// Convert text from polytonic to monotonic Greek.
///
/// Leaves non greek words unchanged.
///
/// ```
/// use grac::*;
///
/// let text = "Ἑλλάς καὶ κόσμος.\r\n...ἄνθρωπος.";
/// let result = to_monotonic(text);
/// assert_eq!(result, "Ελλάς και κόσμος.\r\n...άνθρωπος.");
/// ```
pub fn to_monotonic(s: &str) -> String {
    s.split_inclusive(|ch: char|
        // Split on hyphens (and faulty variations)
        ch == '-' || ch == '—'
        // The main separator logic is whitespace
        || ch.is_whitespace())
        .map(to_monotonic_word)
        .collect()
}

// Uses the is_greek_letter fast path
fn not_punct(ch: char) -> bool {
    is_greek_letter(ch) || (ch != '\u{02BC}' && ch.is_alphabetic())
}

/// Split string into (left_punct, core, right_punct)
///
/// Leaves punctuation inside the core untouched.
pub fn split_punctuation(s: &str) -> (&str, &str, &str) {
    let start = s
        .char_indices()
        .find_map(|(i, ch)| not_punct(ch).then_some(i));

    if let Some(start) = start {
        let end = s
            .char_indices()
            .rev()
            .find_map(|(i, ch)| not_punct(ch).then_some(i + ch.len_utf8()))
            .unwrap();
        (&s[..start], &s[start..end], &s[end..])
    } else {
        // If there is not a single alphabetic char
        // treat the word as left punctuation.
        (s, "", "")
    }
}

/// Monosyllables from which we want to remove the accent.
//
// These have an accent in polytonic that conflicts with our syllabify logic.
// We need to store them separatedly to treat them as monosyllables with no accent.
// Ex. πιὸ ταπεινά > πιό ταπεινά // Expected: πιο ταπεινά
#[rustfmt::skip]
const MONOSYL_REMOVE_ACCENT: [&str; 32] = [
    "πιό", "Πιό", "πιά", "Πιά",
    "μιά", "Μιά", "μιάς", "Μιάς", "γιά", "Γιά", "γειά", "Γειά",
    // πιώ
    "πιώ", "Πιώ", "πίεις", "Πίεις", "πίη", "Πίη", "πιή", "Πιή",
    "πίει", "Πίει", "πιεί", "Πιεί", "πίης", "Πίης", "πιής", "Πιής",
    "πιούν", "Πιούν", "πιές", "Πιές"
];

#[allow(unused_variables)]
fn log(label: &str, value: impl std::fmt::Debug) {
    // println!("{:<30}: {:?}", label, value);
}

fn dbg_bytes(s: &str) {
    log(
        "Input bytes",
        s.as_bytes()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" "),
    );
}

/// Remove ancient diacritics and convert grave and circumflex to acute
/// in a single pass.
///
/// TODO:
/// filter_map was performing a bit worse but remains to be tested.
fn convert_to_acute(s: &str) -> String {
    s.nfd()
        .filter(|ch| {
            ![
                Diacritic::IOTA_SUBSCRIPT,
                Diacritic::ROUGH,
                Diacritic::SMOOTH,
            ]
            .contains(ch)
        })
        .map(|ch| match ch {
            Diacritic::GRAVE | Diacritic::CIRCUMFLEX => Diacritic::ACUTE,
            _ => ch,
        })
        .nfc()
        .collect::<String>()
}

/// Convert a string representing a word to monotonic Greek.
fn to_monotonic_word(s: &str) -> String {
    // If the word is empty our segmentation logic is probably wrong.
    assert!(!s.is_empty());

    // Do not remove accents if the word is not greek
    if !is_greek_word(s) {
        log("Not a greek word!", s);
        return s.to_string();
    }

    // Decompose punctuation
    let (left_punct, core, right_punct) = split_punctuation(s);
    log("Left punct", left_punct);
    log("Right punct", right_punct);

    // Special cases where we need the polytonic word to make a decision:
    // Ex: ποῦ => πού, ποὺ => που
    let ret = match core {
        "ποὺ" => Some("που"),
        "πὼς" => Some("πως"),
        _ => None,
    };
    if let Some(ret) = ret {
        return format!("{}{}{}", left_punct, ret, right_punct);
    }

    log("Input word", core);
    dbg_bytes(core);

    let mut out: String = convert_to_acute(core);

    let ends_with_abbreviation = match right_punct.chars().next() {
        Some(fst_rpunct) => APOSTROPHES.contains(&fst_rpunct),
        None => false,
    };
    log("Ends in abbreviation?", ends_with_abbreviation);

    let syllables = syllabify_el(&out);
    log("Syllabified word", &syllables);

    out = match syllables.as_slice() {
        // Do we remove the acute accent from a monosyllable?...
        [syl] => {
            // To remove the acute, the word should:
            // - not be in the excluded list
            // - not end in an abbreviation mark: έτσ' είναι
            // - not end in a diphthong: σόι, Κάιν etc.
            if !MONOSYLLABLE_ACCENTED.contains(syl)
                && !ends_with_abbreviation
                && !ends_with_diphthong(&out)
            {
                log("Monosyllable no accent", "Removing accents");
                remove_acute(&out)
            } else {
                log("Word keeps accents", &out);
                out
            }
        }
        [.., syl1, syl2] => {
            if MONOSYL_REMOVE_ACCENT.contains(&out.as_str()) {
                log("Word in NOT_ACCENTED list", "Removing accents");
                remove_acute(&out)
            } else if has_acute(*syl1) && has_acute(*syl2) {
                log("Two acute accents in two syllables", "Removing last acute");
                remove_diacritic_at(&out, 1, Diacritic::ACUTE)
            } else {
                log("Word keeps accents", &out);
                out
            }
        }
        _ => out,
    };

    // We do this quite late to deal with Κέϋνς -> two syllables
    // If we did this before splitting on syllables then
    // Κέυνς will only consistute one syllable.
    out = remove_superfluous_diaereses(&out);
    log("Removed superfluous diaereses", &out);

    log("Final transformed word", &out);
    dbg_bytes(&out);
    log("======================", "");

    format!("{}{}{}", left_punct, out, right_punct)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_punct() {
        let non_punct = ['α', 'a', 'ή'];
        for p in non_punct {
            assert!(not_punct(p));
        }

        let punct = ['.', '2', '᾿'];
        for p in punct {
            assert!(!not_punct(p));
        }
    }

    #[test]
    fn test_split_word_punctuation() {
        assert_eq!(split_punctuation("λέξη..."), ("", "λέξη", "..."));
        assert_eq!(split_punctuation(";?λέξη"), (";?", "λέξη", ""));
        assert_eq!(split_punctuation(";?λέξη..."), (";?", "λέξη", "..."));
        assert_eq!(split_punctuation(";?λέ-ξη..."), (";?", "λέ-ξη", "..."));
        assert_eq!(split_punctuation(";?..."), (";?...", "", ""));
        assert_eq!(split_punctuation("2ος"), ("2", "ος", ""));
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
                    let result = to_monotonic(input);
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
        // These require a better syllabify logic...
        // ["χλιός", "χλιος"],
        // ["Δαυίδ", "Δαυίδ"],
        ["δύο-τρεῖς", "δύο-τρεις"],
        // Faulty variation of a hyphen
        ["δύο—τρεῖς", "δύο—τρεις"],
        ["λογιῶν-τῶν-λογιῶν", "λογιών-των-λογιών"],
        ["Ἅμα πιῇς τσάι", "Άμα πιης τσάι"],
    );

    mktest_mono!(
        mono_one_syl,
        ["Πιὸ σιγά, πιὸ ταπεινά", "Πιο σιγά, πιο ταπεινά"],
        ["πιό", "πιο"],
        ["πιά πείς", "πια πεις"],
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
        ["μπέικον", "μπέικον"],
        ["ἄϋλος", "άυλος"],
        ["φαΐ", "φαΐ"],
        ["ἀρχαϊκάς", "αρχαϊκάς"],
        ["Στάυλς", "Στάυλς"],
        ["κακόϋπνος", "κακόυπνος"],
    );

    mktest_mono!(
        mono_not_greek,
        ["1808·", "1808·"],
        ["Poète", "Poète"],
        ["Poète.", "Poète."],
        [".Poète", ".Poète"],
        ["Poè-te", "Poè-te"],
        ["Poè!!te", "Poè!!te"],
        [".Poè-te.", ".Poè-te."],
        ["Le Poète des nuées", "Le Poète des nuées"],
        ["Qui hante la tempête", "Qui hante la tempête"],
        ["Exilé au milieu des huées,", "Exilé au milieu des huées,"],
        ["géant l'empêchent", "géant l'empêchent"],
    );

    mktest_mono!(
        mono_diphthong_in_monosyllable,
        ["σόι", "σόι"],
        ["Γκρέι", "Γκρέι"],
        ["μάι", "μάι"],
        ["Κάιν", "Κάιν"],
        ["Μπόις", "Μπόις"],
        // Rare English transliterations
        ["Φλόυντ", "Φλόυντ"],
        ["Ρόυς", "Ρόυς"],
    );

    // Happens in words that preceed stress-less words: τι, ποτε...
    mktest_mono!(
        mono_double_accents,
        ["εἶναί", "είναι"],
        ["οἶκόν", "οίκον"],
        ["πρᾶγμά", "πράγμα"],
        ["σφαγεῖόν", "σφαγείον"],
        ["νὰ εἶναί τις Βενετός...", "να είναι τις Βενετός..."],
        ["τὸν οἶκόν του μηδὲ...", "τον οίκον του μηδέ..."],
        ["βαστάζοντες πρᾶγμά τι...", "βαστάζοντες πράγμα τι..."],
        ["οἱ ὡραῖοί σου ὀδόντες,", "οι ωραίοι σου οδόντες,"],
    );

    mktest_mono!(
        mono_double_accents_bis,
        ["ἐλαττώματά του", "ελαττώματά του"],
        ["τὸν σύζυγόν της", "τον σύζυγόν της"],
    );

    // Word ending punctuation
    // Do not treat abbreviation as monosyllables
    mktest_mono!(
        mono_ending_punct,
        ["Ὅλ᾽", "Όλ᾽"],
        ["ὅλ᾿ αἱ", "όλ᾿ αι"],
        ["ἔτσ᾿ εἶναι", "έτσ᾿ είναι"],
        ["έτσ' είναι", "έτσ' είναι"],
        ["οὔτ᾿ ἐνθύμηση", "ούτ᾿ ενθύμηση"],
        ["Τί κάν᾽ ἡ λεχώνα;", "Τι κάν᾽ η λεχώνα;"],
        // We can not guess before elipsis: it may be either an actual
        // monosyllable or a truncated word.
        // We decide to go with the removal because it is more likely
        // that the word is not truncated.
        ["πρά…", "πρα…"], // This could be πράσινο...
        ["Ναί… ναί…", "Ναι… ναι…"],
    );

    // Starting punctuation
    mktest_mono!(
        mono_starting_punct,
        ["«τὸ", "«το"],
        ["―Σπαργανίσαμε*", "―Σπαργανίσαμε*"],
        ["διηγῆται:\n\n―\u{2009}Τὴν", "διηγήται:\n\n―\u{2009}Την"],
        ["ἐμάλωσες·", "εμάλωσες·"],
    );

    // Inner punctuation
    mktest_mono!(
        mono_inner_punct,
        // ["Ναί…ναί…", "Ναι…ναι…"],
        // oti variants
        ["ὅ,τι", "ό,τι"],
        ["Ὅ,τι τοῦ τύχῃ", "Ό,τι του τύχη"],
        ["Ό,τ᾿ είναι", "Ό,τ᾿ είναι"],
    );

    mktest_mono!(
        mono_names,
        ["Τζὼν", "Τζων"],
        ["Μέυναρντ", "Μέυναρντ"],
        ["Κέϋνς", "Κέυνς"],
        ["Σὲρ Ἄρθουρ Ἰγνάτιος", "Σερ Άρθουρ Ιγνάτιος"],
        ["Κόναν Ντόϊλ", "Κόναν Ντόιλ"],
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

    mktest_mono!(
        mono_ancient,
        ["ἐξεσάωσεν ὑπʼ ἰλύος", "εξεσάωσεν υπʼ ιλύος"],
        ["ὄφρʼ ἐνὶ", "όφρʼ ενί"],
        ["σῶφρον ἐπʼ ἀρετήν", "σώφρον επʼ αρετήν"],
        ["λήξεν άμʼ ἠελίω· τάχα", "λήξεν άμʼ ηελίω· τάχα"],
    );
}
