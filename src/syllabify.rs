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

/// Locations where synizesis occurs in text.
pub enum Synizesis<'a> {
    Every,
    Never,
    /// A slice of syllable indices where synizesis should occur.
    /// They are 1-indexed counting from the end of the word.
    ///
    /// In case of multiple indices, they should refer to the syllable
    /// positions after the desired synizesis takes place.
    ///
    /// ## Example
    ///
    /// `syllabify_el_mode("αστειάκια", SynizesisAt::Indices(&[1, 2]))`
    /// `vec!["α", "στειά", "κια"]`
    ///
    /// Even though without synizesis the word contains five syllables:
    /// `vec!["α", "στει", "ά", "κι", "α"]`
    ///
    /// and we may be tempted to use `&[1, 3]` to refer to the syllables
    /// that we want to merge.
    Indices(&'a [usize]),
}

pub fn syllabify_gr(word: &str) -> Vec<&str> {
    syllabify_lang(word, &GR, Synizesis::Never)
}

/// Syllabify a modern Greek word.
///
/// Automatically detects synizesis.
///
/// ```
/// use grac::syllabify_el;
/// assert_eq!(syllabify_el("αρρώστια"), vec!["αρ", "ρώ", "στια"]);
/// ```
pub fn syllabify_el(word: &str) -> Vec<&str> {
    match lookup_synizesis(word) {
        Some(res) => res.to_vec(),
        _ => syllabify_lang(word, &EL, Synizesis::Never),
    }
}

/// Syllabify a modern Greek word.
///
/// ```
/// use grac::{syllabify_el_mode, Synizesis};
///
/// assert_eq!(syllabify_el_mode("αρρώστια", Synizesis::Every), vec!["αρ", "ρώ", "στια"]);
/// assert_eq!(syllabify_el_mode("αρρώστια", Synizesis::Never), vec!["αρ", "ρώ", "στι", "α"]);
///
/// // Only apply synizesis at the first syllable from the end.
/// // Note that "στειά" does not merge.
/// let idxs = Synizesis::Indices(&[1]);
/// assert_eq!(syllabify_el_mode("αστειάκια", idxs), vec!["α", "στει", "ά", "κια"]);
///
/// // The indices refer to the syllable positions after the change.
/// let idxs = Synizesis::Indices(&[1, 2]);
/// assert_eq!(syllabify_el_mode("αστειάκια", idxs), vec!["α", "στειά", "κια"]);
/// ```
pub fn syllabify_el_mode<'a>(word: &'a str, synizesis: Synizesis) -> Vec<&'a str> {
    syllabify_lang(word, &EL, synizesis)
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
fn get_byte_offset(pos: usize, chars: &[char]) -> usize {
    chars[..pos].iter().map(|c| c.len_utf8()).sum::<usize>()
}

fn syllabify_lang<'a>(word: &'a str, lang: &Lang, synizesis: Synizesis) -> Vec<&'a str> {
    let chars: Vec<char> = word.chars().collect();
    let mut fr = chars.len();
    let mut fr_byte = get_byte_offset(fr, &chars);
    let mut syllables = Vec::new();
    let mut idx_syllable = 1;

    loop {
        let cur_synizesis = match synizesis {
            Synizesis::Every => true,
            Synizesis::Never => false,
            Synizesis::Indices(idxs) => idxs.contains(&idx_syllable),
        };
        idx_syllable += 1;

        if let Some(to) = parse_syllable_break(&chars, fr, lang, cur_synizesis) {
            let to_byte = get_byte_offset(to, &chars);
            let syllable = &word[to_byte..fr_byte];
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

fn parse_syllable_break(chars: &[char], fr: usize, lang: &Lang, synizesis: bool) -> Option<usize> {
    let mut to = fr;

    move_coda(chars, &mut to, lang);
    move_nucleus(chars, &mut to, lang, synizesis);
    move_onset(chars, &mut to, lang);

    if fr > to {
        Some(to)
    } else {
        None
    }
}

fn move_coda(chars: &[char], pos: &mut usize, lang: &Lang) {
    while *pos > 0 && !is_vowel(chars[*pos - 1], lang) {
        *pos -= 1;
    }
}

fn move_nucleus(chars: &[char], pos: &mut usize, lang: &Lang, synizesis: bool) {
    let to = *pos;
    while *pos > 0 && (is_vowel(chars[*pos - 1], lang) || chars[*pos - 1] == Diacritic::ROUGH) {
        if to - *pos > 0 && chars[*pos] != Diacritic::ACUTE && chars[*pos] != Diacritic::ROUGH {
            if is_diphthong(&chars[*pos - 1..*pos + 1], lang) {
                if to - *pos > 1 && chars.get(*pos + 1) == Some(&'ι') {
                    *pos += 1;
                    break;
                }
            } else if synizesis && chars.get(*pos - 1) == Some(&'ι') {
            } else {
                break;
            }
        }
        *pos -= 1;
    }
}

fn move_onset(chars: &[char], pos: &mut usize, lang: &Lang) {
    let to = *pos;
    while *pos > 0
        && !is_vowel(chars[*pos - 1], lang)
        // If we reach a consonant cluster we keep moving
        && (to == *pos || is_consonant_cluster(&chars[*pos - 1..to], lang))
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
fn dump<'a>(
    chars: &[char],
    fr: usize,
    to: &mut usize,
    result: &mut Vec<&'a str>,
    original: &'a str,
) {
    let start = get_byte_offset(fr, chars);
    let end = get_byte_offset(*to, chars);
    result.push(&original[start..end]);
}

#[inline(always)]
fn dumpmove<'a>(
    chars: &[char],
    fr: usize,
    to: &mut usize,
    result: &mut Vec<&'a str>,
    original: &'a str,
) {
    dump(chars, fr, to, result, original);
    *to = fr;
}

pub fn syllabify_gr_ref(word: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut state = 0;
    let chars: Vec<char> = word.chars().collect();
    let mut to = chars.len();

    for (fr, &ch) in chars.iter().enumerate().rev() {
        match state {
            0 if is_vowel_gr(ch) => state = 1,
            1 => {
                if is_vowel_gr(ch) || ch == Diacritic::ROUGH {
                    let prev = chars[fr + 1];

                    if prev == Diacritic::ACUTE || prev == Diacritic::ROUGH {
                        // Do nothing
                    } else if is_diphthong_gr(&chars[fr..fr + 2]) {
                        // Two consecutive overlapping diphthongs?
                        if chars.get(fr + 2) == Some(&'ι') {
                            // Dump only the part after the iota
                            if fr + 2 < to {
                                dump(&chars, fr + 2, &mut to, &mut result, word);
                                to = fr + 2;
                            }
                        }
                    } else {
                        dumpmove(&chars, fr + 1, &mut to, &mut result, word);
                    }
                } else {
                    state = 2;
                }
            }
            2 => {
                if is_vowel_gr(ch) {
                    dumpmove(&chars, fr + 1, &mut to, &mut result, word);
                    state = 1;
                } else if !is_consonant_cluster_gr(&chars[fr..to]) {
                    dumpmove(&chars, fr + 1, &mut to, &mut result, word);
                    state = 0;
                }
            }
            _ => {}
        }
    }

    if 0 < to {
        dump(&chars, 0, &mut to, &mut result, word);
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
        let chars: Vec<char> = word.chars().collect();
        let pos = chars.len();
        let syllable_break = parse_syllable_break(&chars, pos, &GR, false);
        assert_eq!(syllable_break, Some(0));
    }

    #[test]
    fn test_mia_syllable_syn_true() {
        let word = "μια";
        let chars: Vec<char> = word.chars().collect();
        let pos = chars.len();
        let syllable_break = parse_syllable_break(&chars, pos, &GR, true);
        assert_eq!(syllable_break, Some(0));
    }

    #[test]
    fn test_mia_syllable_syn_false() {
        let word = "μια";
        let chars: Vec<char> = word.chars().collect();
        let pos = chars.len();
        let syllable_break = parse_syllable_break(&chars, pos, &GR, false);
        assert_eq!(syllable_break, Some(2));
    }

    #[test]
    fn test_syllabify_el() {
        let word = "μπεις";
        assert_eq!(syllabify_el(word).len(), 1)
    }

    #[test]
    fn test_is_diphthong() {
        assert!(is_diphthong_gr(&['α', 'ι']));
        assert!(!is_diphthong_gr(&['ι', 'α', 'ι']));
        assert!(!is_diphthong_gr(&['α', 'ε']));
        assert!(!is_diphthong_gr(&['α', 'ϋ']));
    }

    #[test]
    fn test_synizesis_at_simple() {
        assert_eq!(
            syllabify_el_mode("αστειάκια", Synizesis::Indices(&[1])),
            vec!["α", "στει", "ά", "κια"]
        );
    }

    #[test]
    fn test_synizesis_at_multiple() {
        assert_eq!(
            syllabify_el_mode("αστειάκια", Synizesis::Indices(&[1, 2])),
            vec!["α", "στειά", "κια"]
        );
    }
}
