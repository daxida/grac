use crate::accents::{has_diaeresis, Diacritic};
use crate::chars::base_lower;
use crate::synizesis::lookup_synizesis;

// By frequency: https://www.sttmedia.com/characterfrequency-greek
#[rustfmt::skip]
const VOWELS_GR: [char; 9] = [
    'α', 'ο', 'ε', 'ι', 'η', 'υ', 'ω',
    '~', Diacritic::ACUTE,
];

const DIPHTHONGS_GR: [(char, char); 8] = [
    ('α', 'ι'),
    ('ε', 'ι'),
    ('ο', 'ι'),
    ('υ', 'ι'),
    ('α', 'υ'),
    ('ε', 'υ'),
    ('ο', 'υ'),
    ('η', 'υ'),
];

#[rustfmt::skip]
const CONS_CLUSTERS_GR: [(char, char); 33] = [
    ('β', 'δ'), ('β', 'λ'), ('β', 'ρ'),
    ('γ', 'λ'), ('γ', 'ν'), ('γ', 'ρ'),
    ('δ', 'ρ'),
    ('θ', 'λ'), ('θ', 'ν'), ('θ', 'ρ'),
    ('κ', 'λ'), ('κ', 'ν'), ('κ', 'ρ'), ('κ', 'τ'),
    ('μ', 'ν'),
    ('π', 'λ'), ('π', 'ν'), ('π', 'ρ'), ('π', 'τ'),
    ('σ', 'β'), ('σ', 'θ'), ('σ', 'κ'), ('σ', 'μ'), ('σ', 'π'), ('σ', 'τ'), ('σ', 'φ'), ('σ', 'χ'),
    ('τ', 'ρ'),
    ('φ', 'θ'), ('φ', 'λ'), ('φ', 'ρ'),
    ('χ', 'λ'), ('χ', 'ρ'),
];

#[rustfmt::skip]
const VOWELS_EL: [char; 7] = [
    'α', 'ο', 'ε', 'ι', 'η', 'υ', 'ω',
];

const DIPHTHONGS_EL: [(char, char); 8] = [
    ('α', 'ι'),
    ('ε', 'ι'),
    ('ο', 'ι'),
    ('α', 'υ'),
    ('ε', 'υ'),
    ('ο', 'υ'),
    ('η', 'υ'),
    ('υ', 'ι'),
];

#[rustfmt::skip]
const CONS_CLUSTERS_EL: [(char, char); 45] = [
    ('β', 'δ'), ('β', 'λ'), ('β', 'ρ'), ('β', 'γ'),
    ('γ', 'κ'), ('γ', 'λ'), ('γ', 'ν'), ('γ', 'ρ'),
    ('δ', 'ρ'),
    ('θ', 'λ'), ('θ', 'ν'), ('θ', 'ρ'),
    ('κ', 'λ'), ('κ', 'ν'), ('κ', 'ρ'), ('κ', 'τ'),
    ('μ', 'ν'), ('μ', 'π'),
    ('ν', 'τ'),
    ('π', 'λ'), ('π', 'ν'), ('π', 'ρ'), ('π', 'τ'),
    ('σ', 'β'), ('σ', 'θ'), ('σ', 'κ'), ('σ', 'τ'), ('σ', 'φ'), ('σ', 'χ'), ('σ', 'μ'), ('σ', 'π'),
    ('τ', 'μ'), ('τ', 'ρ'), ('τ', 'ζ'), ('τ', 'σ'), ('τ', 'λ'),
    ('φ', 'θ'), ('φ', 'λ'), ('φ', 'ρ'), ('φ', 'τ'),
    ('χ', 'λ'), ('χ', 'ρ'), ('χ', 'θ'), ('χ', 'τ'), ('χ', 'ν'),
];

struct Lang<'a> {
    vowels: &'a [char],
    diphthongs: &'a [(char, char)],
    cons_clusters: &'a [(char, char)],
}

const GR: Lang = Lang {
    vowels: &VOWELS_GR,
    diphthongs: &DIPHTHONGS_GR,
    cons_clusters: &CONS_CLUSTERS_GR,
};

const EL: Lang = Lang {
    vowels: &VOWELS_EL,
    diphthongs: &DIPHTHONGS_EL,
    cons_clusters: &CONS_CLUSTERS_EL,
};

/// Locations to merge syllables at vowels.
///
/// # `Merge::Indices`
///
/// A slice of syllable indices where merging should occur.
/// They are 1-indexed counting from the end of the word.
///
/// In case of multiple indices, they should refer to the syllable
/// positions after the desired merging takes place.
///
/// # Example
///
/// ```
/// use grac::{syllabify_el_mode, Merge};
///
/// let result = syllabify_el_mode("αστειάκια", Merge::Indices(&[1, 2]));
/// assert_eq!(result, vec!["α", "στειά", "κια"]);
/// // Even though without merge the word contains five syllables:
/// // `vec!["α", "στει", "ά", "κι", "α"]`
/// // and we may be tempted to use `&[1, 3]` to refer to the syllables
/// // that we want to merge.
/// ```
pub enum Merge<'a> {
    Every,
    Never,
    Indices(&'a [usize]),
}

pub fn syllabify_gr(s: &str) -> Vec<&str> {
    syllabify_lang(s, &GR, Merge::Never)
}

/// Syllabify a modern Greek word.
///
/// Automatically detects synizesis.
///
/// # Example
///
/// ```
/// use grac::syllabify_el;
/// assert_eq!(syllabify_el("αρρώστια"), vec!["αρ", "ρώ", "στια"]);
/// ```
pub fn syllabify_el(s: &str) -> Vec<&str> {
    match lookup_synizesis(s) {
        Some(res) => res.to_vec(),
        _ => syllabify_lang(s, &EL, Merge::Never),
    }
}

/// Syllabify a modern Greek word.
///
/// # Example
///
/// ```
/// use grac::{syllabify_el_mode, Merge};
///
/// assert_eq!(syllabify_el_mode("αρρώστια", Merge::Every), vec!["αρ", "ρώ", "στια"]);
/// assert_eq!(syllabify_el_mode("αρρώστια", Merge::Never), vec!["αρ", "ρώ", "στι", "α"]);
///
/// // Merge only at the first syllable from the end.
/// // Note that "στειά" does not merge.
/// let idxs = Merge::Indices(&[1]);
/// assert_eq!(syllabify_el_mode("αστειάκια", idxs), vec!["α", "στει", "ά", "κια"]);
///
/// // The indices refer to the syllable positions after the change.
/// let idxs = Merge::Indices(&[1, 2]);
/// assert_eq!(syllabify_el_mode("αστειάκια", idxs), vec!["α", "στειά", "κια"]);
///
/// // Merge only words for vowel frontiers, and from back to start.
/// assert_eq!(syllabify_el_mode("χέρια", Merge::Indices(&[1])), vec!["χέ", "ρια"]);
/// assert_eq!(syllabify_el_mode("χέρια", Merge::Indices(&[2])), vec!["χέ", "ρι", "α"]);
/// assert_eq!(syllabify_el_mode("χέρια", Merge::Indices(&[3])), vec!["χέ", "ρι", "α"]);
/// ```
pub fn syllabify_el_mode<'a>(s: &'a str, merge: Merge) -> Vec<&'a str> {
    syllabify_lang(s, &EL, merge)
}

/////////////////////////////////////////////

fn is_vowel(ch: char, lang: &Lang) -> bool {
    lang.vowels.contains(&base_lower(ch))
}

fn is_diphthong(chs: &[char], lang: &Lang) -> bool {
    match chs {
        [a, b] => {
            let pair = (base_lower(*a), base_lower(*b));
            lang.diphthongs.contains(&pair) && !has_diaeresis(*b)
        }
        _ => false,
    }
}

fn is_consonant_cluster(chs: &[char], lang: &Lang) -> bool {
    match chs {
        [a, b, ..] => {
            let pair = (base_lower(*a), base_lower(*b));
            lang.cons_clusters.contains(&pair)
        }
        _ => false,
    }
}

#[inline]
fn get_byte_offset(pos: usize, chs: &[char]) -> usize {
    chs[..pos].iter().map(|ch| ch.len_utf8()).sum::<usize>()
}

fn syllabify_lang<'a>(s: &'a str, lang: &Lang, merge: Merge) -> Vec<&'a str> {
    let chs: Vec<_> = s.chars().collect();
    let mut fr = chs.len();
    let mut fr_byte = get_byte_offset(fr, &chs);
    let mut syllables = Vec::new();
    let mut idx_syllable = 1;

    loop {
        let cur_merge = match merge {
            Merge::Every => true,
            Merge::Never => false,
            Merge::Indices(idxs) => idxs.contains(&idx_syllable),
        };
        idx_syllable += 1;

        if let Some(to) = parse_syllable_break(&chs, fr, lang, cur_merge) {
            let to_byte = get_byte_offset(to, &chs);
            let syllable = &s[to_byte..fr_byte];
            syllables.push(syllable);
            fr = to;
            fr_byte = to_byte;
        } else {
            break;
        }
    }

    syllables.reverse();
    syllables
}

fn parse_syllable_break(chs: &[char], fr: usize, lang: &Lang, merge: bool) -> Option<usize> {
    let mut to = fr;

    move_coda(chs, &mut to, lang);
    move_nucleus(chs, &mut to, lang, merge);
    move_onset(chs, &mut to, lang);

    if fr > to {
        Some(to)
    } else {
        None
    }
}

fn move_coda(chs: &[char], pos: &mut usize, lang: &Lang) {
    while *pos > 0 && !is_vowel(chs[*pos - 1], lang) {
        *pos -= 1;
    }
}

const CANDIDATE_MERGING_DIPHTHONGS_EL: [(char, char); 10] = [
    ('α', 'η'),
    ('ά', 'η'),
    ('α', 'ϊ'),
    ('α', 'ΐ'),
    ('ά', 'ι'),
    ('ο', 'η'),
    ('ό', 'η'),
    ('ο', 'ϊ'),
    ('ο', 'ΐ'),
    ('ό', 'ι'),
];

fn is_candidate_diphthong(chs: &[char]) -> bool {
    match chs {
        [a, b] => CANDIDATE_MERGING_DIPHTHONGS_EL.contains(&(*a, *b)),
        _ => false,
    }
}

fn is_candidate_synizesis(chs: &[char], pos: usize) -> bool {
    matches!(chs.get(pos - 1), Some('ι' | 'υ' | 'η'))
}

fn move_nucleus(chs: &[char], pos: &mut usize, lang: &Lang, merge: bool) {
    let to = *pos;
    while *pos > 0 && (is_vowel(chs[*pos - 1], lang) || chs[*pos - 1] == Diacritic::ROUGH) {
        if to - *pos > 0 && chs[*pos] != Diacritic::ACUTE && chs[*pos] != Diacritic::ROUGH {
            let icd = is_candidate_diphthong(&chs[*pos - 1..*pos + 1]);

            if merge && (is_candidate_synizesis(chs, *pos) || icd) {
                // Keep moving
            } else if !icd && is_diphthong(&chs[*pos - 1..*pos + 1], lang) {
                // Deal with overlapping diphthongs: ουι
                if to - *pos > 1 && chs.get(*pos + 1) == Some(&'ι') {
                    *pos += 1;
                    break;
                }
                // Keep moving
            } else {
                break;
            }
        }
        *pos -= 1;
    }
}

fn move_onset(chs: &[char], pos: &mut usize, lang: &Lang) {
    let to = *pos;
    while *pos > 0
        && !is_vowel(chs[*pos - 1], lang)
        // If we reach a consonant cluster we keep moving
        && (to == *pos || is_consonant_cluster(&chs[*pos - 1..to], lang))
    {
        *pos -= 1;
    }
}

///////////// Oracle reference. Not intended for use.

pub fn is_vowel_el(ch: char) -> bool {
    is_vowel(ch, &EL)
}

fn is_vowel_gr(ch: char) -> bool {
    is_vowel(ch, &GR)
}

fn is_diphthong_gr(chs: &[char]) -> bool {
    is_diphthong(chs, &GR)
}

fn is_consonant_cluster_gr(chs: &[char]) -> bool {
    is_consonant_cluster(chs, &GR)
}

#[inline(always)]
fn dump<'a>(chs: &[char], fr: usize, to: &usize, result: &mut Vec<&'a str>, original: &'a str) {
    let start = get_byte_offset(fr, chs);
    let end = get_byte_offset(*to, chs);
    result.push(&original[start..end]);
}

#[derive(Debug)]
enum State {
    Start,
    FoundVowel,
    FoundConsonant,
}

#[inline(always)]
fn dumpmove<'a>(
    chs: &[char],
    fr: usize,
    to: &mut usize,
    result: &mut Vec<&'a str>,
    original: &'a str,
) {
    dump(chs, fr, to, result, original);
    *to = fr;
}

pub fn syllabify_gr_ref(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut state = State::Start;
    let chs: Vec<_> = s.chars().collect();
    let mut to = chs.len();

    for (fr, &ch) in chs.iter().enumerate().rev() {
        match state {
            State::Start => {
                if is_vowel_gr(ch) {
                    state = State::FoundVowel;
                }
            }
            State::FoundVowel => {
                if is_vowel_gr(ch) || ch == Diacritic::ROUGH {
                    let prev = chs[fr + 1];

                    if prev == Diacritic::ACUTE || prev == Diacritic::ROUGH {
                        // Do nothing
                    } else if !is_candidate_diphthong(&chs[fr..fr + 2])
                        && is_diphthong_gr(&chs[fr..fr + 2])
                    {
                        // Two consecutive overlapping diphthongs?
                        if chs.get(fr + 2) == Some(&'ι') {
                            // Dump only the part after the iota
                            if fr + 2 < to {
                                dump(&chs, fr + 2, &to, &mut result, s);
                                to = fr + 2;
                            }
                        }
                    } else {
                        dumpmove(&chs, fr + 1, &mut to, &mut result, s);
                    }
                } else {
                    state = State::FoundConsonant;
                }
            }
            State::FoundConsonant => {
                if is_vowel_gr(ch) {
                    dumpmove(&chs, fr + 1, &mut to, &mut result, s);
                    state = State::FoundVowel;
                } else if !is_consonant_cluster_gr(&chs[fr..to]) {
                    dumpmove(&chs, fr + 1, &mut to, &mut result, s);
                    state = State::Start;
                }
            }
        }
    }

    if 0 < to {
        dump(&chs, 0, &to, &mut result, s);
    }

    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_syllable() {
        let word = "στρες";
        let chs: Vec<_> = word.chars().collect();
        let pos = chs.len();
        let syllable_break = parse_syllable_break(&chs, pos, &GR, false);
        assert_eq!(syllable_break, Some(0));
    }

    #[test]
    fn test_mia_syllable_syn_true() {
        let word = "μια";
        let chs: Vec<_> = word.chars().collect();
        let pos = chs.len();
        let syllable_break = parse_syllable_break(&chs, pos, &GR, true);
        assert_eq!(syllable_break, Some(0));
    }

    #[test]
    fn test_mia_syllable_syn_false() {
        let word = "μια";
        let chs: Vec<_> = word.chars().collect();
        let pos = chs.len();
        let syllable_break = parse_syllable_break(&chs, pos, &GR, false);
        assert_eq!(syllable_break, Some(2));
    }

    #[test]
    fn test_syllabify_el() {
        assert_eq!(syllabify_el("μπεις").len(), 1);
    }

    #[test]
    fn test_is_diphthong() {
        assert!(is_diphthong_gr(&['α', 'ι']));
        assert!(!is_diphthong_gr(&['ι', 'α', 'ι']));
        assert!(!is_diphthong_gr(&['α', 'ε']));
        assert!(!is_diphthong_gr(&['α', 'ϋ']));
    }

    #[test]
    fn test_merge_at_simple() {
        assert_eq!(
            syllabify_el_mode("αστειάκια", Merge::Indices(&[1])),
            vec!["α", "στει", "ά", "κια"]
        );
    }

    #[test]
    fn test_merge_at_multiple() {
        assert_eq!(
            syllabify_el_mode("αστειάκια", Merge::Indices(&[1, 2])),
            vec!["α", "στειά", "κια"]
        );
    }
}
