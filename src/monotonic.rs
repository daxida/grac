use aho_corasick::AhoCorasick;
use unicode_normalization::UnicodeNormalization;

use crate::accents::Diacritic;
use crate::accents::{has_acute, remove_acute, remove_diacritic_at};
use crate::chars::{ends_with_diphthong, is_greek_word};
use crate::constants::{APOSTROPHES, MONOSYLLABLE_ACCENTED};
use crate::is_greek_letter;
use crate::syllabify::syllabify;

// Intended to be run over the entire text, and not individual words.
// When ran over words, the cost of building the automata is too big,
fn remove_superfluous_diaereses(s: &str) -> String {
    const DIAERESES_WRONG: [&str; 14] =
        with_capitalized!(["άϊ", "άϋ", "έϊ", "έϋ", "όϊ", "όϋ", "ούϊ"]);
    const DIAERESES_CORRECT: [&str; 14] =
        with_capitalized!(["άι", "άυ", "έι", "έυ", "όι", "όυ", "ούι"]);
    let ac = AhoCorasick::new(DIAERESES_WRONG).unwrap();
    ac.replace_all(s, &DIAERESES_CORRECT)
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
    let out: String = s
        .split_inclusive(|ch: char|
        // Split on hyphens (and faulty variations)
        ch == '-' || ch == '—'
        // The main separator logic is whitespace
        || ch.is_whitespace())
        .map(to_monotonic_word)
        .collect();
    remove_superfluous_diaereses(&out)
}

// Uses the is_greek_letter fast path
fn not_punct(ch: char) -> bool {
    is_greek_letter(ch) || (ch != '\u{02BC}' && ch.is_alphabetic())
}

/// Split string into (left punctuation, core, right punctuation).
///
/// Leaves punctuation inside the core untouched.
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::option_if_let_else)]
pub fn split_punctuation(s: &str) -> (&str, &str, &str) {
    let start = s
        .char_indices()
        .find_map(|(i, ch)| not_punct(ch).then_some(i));

    if let Some(start) = start {
        // SAFETY: since we found some index "start" from the right, there
        // must be some index (potentially start iself) from the left.
        let end = s
            .char_indices()
            .rev()
            .find_map(|(i, ch)| not_punct(ch).then_some(i + ch.len_utf8()))
            .unwrap();
        (&s[..start], &s[start..end], &s[end..])
    } else {
        // If there are not alphabetic chars, treat the word as left punctuation.
        (s, "", "")
    }
}

/// Monosyllables from which we want to remove the accent.
///
/// These have an accent in polytonic that conflicts with our syllabify logic.
/// We need to store them separatedly to treat them as monosyllables with no accent.
/// Ex. πιὸ ταπεινά > πιό ταπεινά // Expected: πιο ταπεινά
#[rustfmt::skip]
const MONOSYL_REMOVE_ACCENT: [&str; 32] = with_capitalized!([
    "πιό", "πιά", "μιά", "μιάς", "γιά", "γειά",
    // πιώ
    "πιώ",
    "πίεις", "πίη", "πιή", "πίει", "πιεί", "πίης", "πιής",
    "πιούν", "πιές",
]);

#[allow(unused_variables)]
fn log(label: &str, value: impl std::fmt::Debug) {
    // println!("{:<30}: {:?}", label, value);
}

fn dbg_bytes(s: &str) {
    log(
        "Input bytes",
        s.as_bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<Vec<String>>()
            .join(" "),
    );
}

/// Remove ancient diacritics and convert grave and circumflex to acute in a single pass.
fn convert_to_acute(s: &str) -> String {
    const DIACRITICS_TO_REMOVE: [char; 3] = [
        Diacritic::IOTA_SUBSCRIPT,
        Diacritic::ROUGH,
        Diacritic::SMOOTH,
    ];

    s.nfd()
        .filter_map(|ch| {
            if DIACRITICS_TO_REMOVE.contains(&ch) {
                None
            } else if matches!(ch, Diacritic::GRAVE | Diacritic::CIRCUMFLEX) {
                Some(Diacritic::ACUTE)
            } else {
                Some(ch)
            }
        })
        .nfc()
        .collect::<String>()
}

/// Special cases.
///
/// Sometimes we need the polytonic word to make a decision:
/// Ex: ποῦ => πού, ποὺ => που
///
/// Sometimes, due to synizesis, the words with replaced acute accents
/// does not exist:
/// Ex: ποιὸς => ποιός (when it should be ποιος)
fn special_cases(s: &str) -> Option<&str> {
    match s {
        "ποὺ" => Some("που"),
        "Ποὺ" => Some("Που"),
        "πὼς" => Some("πως"),
        "Πὼς" => Some("Πως"),
        // TODO: finish as we find other the cases to test against
        "ποιὸς" => Some("ποιος"),
        "Ποιὸς" => Some("Ποιος"),
        "ποιὰ" => Some("ποια"),
        "Ποιὰ" => Some("Ποια"),
        _ => None,
    }
}

/// Convert a string representing a word to monotonic Greek.
fn to_monotonic_word(s: &str) -> String {
    // If the word is empty our segmentation logic is probably wrong.
    debug_assert!(!s.is_empty());

    // Do not remove accents if the word is not greek
    if !is_greek_word(s) {
        log("Not a greek word!", s);
        return s.to_string();
    }

    // Decompose punctuation
    let (left_punct, core, right_punct) = split_punctuation(s);
    log("Left punct", left_punct);
    log("Right punct", right_punct);

    if let Some(ret) = special_cases(core) {
        return format!("{left_punct}{ret}{right_punct}");
    }

    log("Input word", core);
    dbg_bytes(core);

    let mut out: String = convert_to_acute(core);

    let ends_with_abbreviation = right_punct
        .chars()
        .next()
        .is_some_and(|fst_rpunct| APOSTROPHES.contains(&fst_rpunct));

    log("Ends in abbreviation?", ends_with_abbreviation);

    let syllables = syllabify(&out);
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

    log("Final transformed word", &out);
    dbg_bytes(&out);
    log("======================", "");

    format!("{left_punct}{out}{right_punct}")
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
        [
            "βρωμόσκυλο! Ποὺ βρωμᾷς πρωὶ-πρωὶ",
            "βρωμόσκυλο! Που βρωμάς πρωί-πρωί"
        ]
    );

    mktest_mono!(
        mono_poios_variants,
        ["καὶ ποιὸς τὸν ἐσκότωσε;", "και ποιος τον εσκότωσε;"],
        ["Ποιὸς λοιπὸν τὸν ἐσκότωσε;", "Ποιος λοιπόν τον εσκότωσε;"],
        ["Ἤξερες ποιὰ ἦτον αὐτή;", "Ήξερες ποια ήτον αυτή;"],
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
        ["Άϊντε", "Άιντε"],
        ["Όϊπεν", "Όιπεν"],
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
