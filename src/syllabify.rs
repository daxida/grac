use std::ops::{Deref, DerefMut};

use crate::accents::{Diacritic, has_diaeresis};
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
/// assert_eq!(result.join("-"), "α-στειά-κια");
/// // Even though without merge the word contains five syllables ("α-στει-ά-κι-α")
/// // and we may be tempted to use `&[1, 3]` to refer to the syllables positions.
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Merge<'a> {
    Every,
    Never,
    Indices(&'a [u8]),
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
/// assert_eq!(syllabify_el("αρρώστια").join("-"), "αρ-ρώ-στια");
/// ```
#[allow(clippy::option_if_let_else)]
pub fn syllabify_el(s: &str) -> Syllables<'_> {
    match lookup_synizesis(s) {
        Some(res) => Syllables::from(res),
        _ => syllabify_el_ref(s, Merge::Never),
    }
}

/// Syllabify a modern Greek word.
///
/// # Example
///
/// ```
/// use grac::{syllabify_el_mode, Merge};
///
/// let word = "αστειάκια";
/// assert_eq!(syllabify_el_mode(word, Merge::Every).join("-"), "α-στειά-κια");
/// assert_eq!(syllabify_el_mode(word, Merge::Never).join("-"), "α-στει-ά-κι-α");
///
/// // Merge at the first syllable from the end.
/// let idxs = Merge::Indices(&[1]);
/// assert_eq!(syllabify_el_mode(word, idxs).join("-"), "α-στει-ά-κια");
///
/// // Indices refer to syllable positions after each merge.
/// let idxs = Merge::Indices(&[1, 2]);
/// assert_eq!(syllabify_el_mode(word, idxs).join("-"), "α-στειά-κια");
/// ```
pub fn syllabify_el_mode<'a>(s: &'a str, merge: Merge<'a>) -> Syllables<'a> {
    // syllabify_lang(s, &EL, merge)
    syllabify_el_ref(s, merge)
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

    if fr > to { Some(to) } else { None }
}

fn move_coda(chs: &[char], pos: &mut usize, lang: &Lang) {
    while *pos > 0 && !is_vowel(chs[*pos - 1], lang) {
        *pos -= 1;
    }
}

// For completion it contains:
// * archaic versions: άϊ, όϊ etc.
// * υι, even though this should be (probably!) always unmerged
const CANDIDATE_MERGING_DIPHTHONGS_EL: [(char, char); 19] = [
    // ai
    ('α', 'η'),
    ('ά', 'η'),
    ('α', 'ϊ'),
    ('α', 'ΐ'),
    ('ά', 'ι'),
    ('ά', 'ϊ'),
    // oi
    ('ο', 'η'),
    ('ό', 'η'),
    ('ο', 'ϊ'),
    ('ο', 'ΐ'),
    ('ό', 'ι'),
    ('ό', 'ϊ'),
    // ui (covers υι and ουι)
    ('υ', 'η'),
    ('ύ', 'η'),
    ('υ', 'ϊ'),
    ('υ', 'ΐ'),
    ('ύ', 'ι'),
    ('ύ', 'ϊ'),
    // Needs the extra υί to deal with ουί
    ('υ', 'ί'),
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

#[allow(clippy::range_plus_one)]
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

pub fn is_diphthong_el(chs: &[char]) -> bool {
    is_diphthong(chs, &EL)
}

// To delete once the testing is over
#[allow(dead_code)]
fn is_consonant_cluster_el(chs: &[char]) -> bool {
    is_consonant_cluster(chs, &EL)
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

#[derive(Debug)]
enum State {
    Start,
    FoundVowel,
    FoundConsonant,
}

pub fn syllabify_gr_ref(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
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
                                dumpmove(&chs, fr + 2, &mut to, &mut out, s);
                            }
                        }
                    } else {
                        dumpmove(&chs, fr + 1, &mut to, &mut out, s);
                    }
                } else {
                    state = State::FoundConsonant;
                }
            }
            State::FoundConsonant => {
                if is_vowel_gr(ch) {
                    dumpmove(&chs, fr + 1, &mut to, &mut out, s);
                    state = State::FoundVowel;
                } else if !is_consonant_cluster_gr(&chs[fr..to]) {
                    dumpmove(&chs, fr + 1, &mut to, &mut out, s);
                    state = State::Start;
                }
            }
        }
    }

    if 0 < to {
        dumpmove(&chs, 0, &mut to, &mut out, s);
    }

    out.reverse();
    out
}

#[inline]
fn dumpmove<'a>(chs: &[char], fr: usize, to: &mut usize, out: &mut Vec<&'a str>, src: &'a str) {
    let start = get_byte_offset(fr, chs);
    let end = get_byte_offset(*to, chs);
    out.push(&src[start..end]);
    *to = fr;
}

#[rustfmt::skip]
const fn is_cons_el_opt(ch: char) -> bool {
    matches!(
        ch,
        // lowercase consonants
        'β' | 'γ' | 'δ' | 'ζ' | 'θ' | 'κ' | 'λ' | 'μ' | 'ν' | 'ξ' | 'π' | 'ρ' | 'σ' | 'ς' | 'τ' | 'φ' | 'χ' | 'ψ' |

        // uppercase consonants
        'Β' | 'Γ' | 'Δ' | 'Ζ' | 'Θ' | 'Κ' | 'Λ' | 'Μ' | 'Ν' | 'Ξ' | 'Π' | 'Ρ' | 'Σ' | 'Τ' | 'Φ' | 'Χ' | 'Ψ'
    )
}

// Return true if ch normalizes to a vowel.
//
// Note that it can also return true when ch does not normalize to a vowel.
// mainly for characters we don't care about in the Greek unicode ranges (ex. Ͷ).
const fn is_vowel_el_opt(ch: char) -> bool {
    match ch {
        '\u{0370}'..='\u{03FF}' => {
            // Trying to match vowels is slower, even when sorted by frequency!
            !is_cons_el_opt(ch)
        }
        '\u{1F00}'..='\u{1FFF}' => !matches!(ch, 'ῤ' | 'ῥ' | 'Ῥ'),
        _ => false,
    }
}

#[inline]
fn merge_to_bool(idx_syllable: u8, merge: &Merge) -> bool {
    match merge {
        Merge::Every => true,
        Merge::Never => false,
        Merge::Indices(idxs) => idxs.contains(&idx_syllable),
    }
}

// Rewrite this to not use chars.collect()
pub fn _syllabify_el_ref<'a>(s: &'a str, merge: Merge) -> Vec<&'a str> {
    // Experimentally, it was found:
    // * (number of syllables, count)
    // * [(1, 25150), (2, 16422), (3, 12891), (4, 8869), (5, 3245), (6, 280), (7, 35)]
    let mut out = Vec::with_capacity(8);

    let mut state = State::Start;
    let chs: Vec<_> = s.chars().collect(); // Can skip this?
    let mut to = chs.len();
    let mut to_byte = s.len();

    let mut idx_syllable = 1;
    let mut cur_merge = merge_to_bool(idx_syllable, &merge);

    macro_rules! dump_at {
        ($fr:expr) => {
            let fr_byte = get_byte_offset($fr, &chs);
            out.push(&s[fr_byte..to_byte]);
            to_byte = fr_byte;
            to = $fr;
            // eprintln!("cur_merge {cur_merge:?}");
            // eprintln!("{out:?}");
            idx_syllable += 1;
            cur_merge = merge_to_bool(idx_syllable, &merge);
        };
    }

    for (fr, &ch) in chs.iter().enumerate().rev() {
        let vowel = is_vowel_el_opt(ch);

        // eprintln!("* {:<15} {} {} {}", format!("{state:?}"), fr, ch, vowel);

        match state {
            State::Start => {
                if vowel {
                    state = State::FoundVowel;
                }
            }
            State::FoundVowel => {
                if vowel {
                    let icd = is_candidate_diphthong(&chs[fr..fr + 2]);
                    if cur_merge && (matches!(ch, 'ι' | 'υ' | 'η') || icd) {
                        // Do nothing: advance (i.e. merge)
                    } else if !icd && is_diphthong_el(&chs[fr..fr + 2]) {
                        // Two consecutive overlapping diphthongs? (rare)
                        // -- only ουι (ου / υι are diphthongs)
                        if chs.get(fr + 2) == Some(&'ι') {
                            dump_at!(fr + 2);
                        }
                        // Do nothing: advance (found a diphthong)
                    } else {
                        dump_at!(fr + 1);
                    }
                } else {
                    state = State::FoundConsonant;
                }
            }
            State::FoundConsonant => {
                if vowel {
                    dump_at!(fr + 1);
                    state = State::FoundVowel;
                } else if !is_consonant_cluster_el(&chs[fr..to]) {
                    dump_at!(fr + 1);
                    state = State::Start;
                }
            }
        }
    }

    if 0 < to_byte {
        // Simple dump_at! knowing that fr is 0
        out.push(&s[0..to_byte]);
    }

    out.reverse();
    out
}

fn is_consonant_cluster_el_opt(a: char, b: char) -> bool {
    let pair = (base_lower(a), base_lower(b));
    CONS_CLUSTERS_EL.contains(&pair)
}

type S<'a> = &'a str;
// type Ty<'a> = smallvec::SmallVec<[S<'a>; 8]>;
type Ty<'a> = Vec<S<'a>>;

// Wrapper type to allow for internal experimentation with Vec and SmallVec
#[derive(Debug, PartialEq, Eq)]
pub struct Syllables<'a> {
    inner: Ty<'a>,
}

impl Syllables<'_> {
    pub fn as_slice(&self) -> &[S] {
        self.inner.as_slice()
    }
}

impl<'a> FromIterator<S<'a>> for Syllables<'a> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Syllables {
            inner: Ty::from_iter(iter),
        }
    }
}

impl<'a> Deref for Syllables<'a> {
    type Target = [S<'a>];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Syllables<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// For converting the phf result to Syllables
impl<'a> From<&[&'a str]> for Syllables<'a> {
    fn from(slice: &[&'a str]) -> Self {
        slice.iter().copied().collect()
    }
}

// Because we yield syllables in reverse order, we cannot refactor this into
// returning only an iterator (we need to allocate the syllables somehow).
pub fn syllabify_el_ref<'a>(s: &'a str, merge: Merge) -> Syllables<'a> {
    // Experimentally, it was found:
    // * (number of syllables, count)
    // * [(1, 25150), (2, 16422), (3, 12891), (4, 8869), (5, 3245), (6, 280), (7, 35)]
    let mut out = Ty::with_capacity(8);

    let mut state = State::Start;
    let mut idx_syllable = 1;
    let mut cur_merge = merge_to_bool(idx_syllable, &merge);

    // We'll walk backwards using char_indices().rev(), and buffer recent chars
    let mut to_byte = s.len();
    let mut buffer: [(usize, char); 3] = [(0, '\0'); 3]; // for peeking ahead
    let mut buf_len = 0;

    macro_rules! dump_at {
        ($fr_byte:expr) => {{
            out.push(&s[$fr_byte..to_byte]);
            to_byte = $fr_byte;
            idx_syllable += 1;
            cur_merge = merge_to_bool(idx_syllable, &merge);
        }};
    }

    for (fr_byte, ch) in s.char_indices().rev() {
        // Slide buffer
        buffer.copy_within(0..2, 1);
        buffer[0] = (fr_byte, ch);
        buf_len = buf_len.min(2) + 1;

        let vowel = is_vowel_el_opt(ch);

        // eprintln!(
        //     "* {:<15} {} {} {}",
        //     format!("{state:?}"),
        //     fr_byte,
        //     ch,
        //     vowel
        // );

        match state {
            State::Start => {
                if vowel {
                    state = State::FoundVowel;
                }
            }
            State::FoundVowel => {
                if vowel {
                    debug_assert!(buf_len >= 2);
                    let (next_idx, next_ch) = buffer[1];
                    let icd = is_candidate_diphthong(&[ch, next_ch]);
                    if cur_merge && (matches!(ch, 'ι' | 'υ' | 'η') || icd) {
                        // merge
                    } else if !icd && is_diphthong_el(&[ch, next_ch]) {
                        let (after_next_idx, after_next_ch) = buffer[2];
                        if after_next_ch == 'ι' {
                            debug_assert!(buf_len >= 3);
                            dump_at!(after_next_idx);
                        }
                        // merge
                    } else {
                        dump_at!(next_idx);
                    }
                } else {
                    state = State::FoundConsonant;
                }
            }
            State::FoundConsonant => {
                let (next_idx, next_ch) = buffer[1];
                if vowel {
                    dump_at!(next_idx);
                    state = State::FoundVowel;
                } else if !is_consonant_cluster_el_opt(ch, next_ch) {
                    dump_at!(next_idx);
                    state = State::Start;
                }
                // keep advancing
            }
        }
    }

    if to_byte > 0 {
        out.push(&s[..to_byte]);
    }

    out.reverse();

    Syllables { inner: out }
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
    fn test_is_diphthong() {
        assert!(is_diphthong_gr(&['α', 'ι']));
        assert!(!is_diphthong_gr(&['α', 'ε']));
        assert!(!is_diphthong_gr(&['α', 'ϋ']));
    }

    #[test]
    fn test_syllabify_el() {
        assert_eq!(syllabify_el("μπεις").len(), 1);
        assert_eq!(syllabify_el("Ώστε").len(), 2);
    }

    #[test]
    fn test_syllabify_el_consonant_cluster1() {
        assert_eq!(syllabify_el("στρες").len(), 1);
        assert_eq!(syllabify_el("ΣΤΡΕΣ").len(), 1);
    }

    #[test]
    fn test_syllabify_el_consonant_cluster2() {
        assert_eq!(syllabify_el("χλόη").len(), 2);
    }

    #[test]
    fn test_syllabify_diaeresis() {
        assert_eq!(syllabify_el("φαΐ").len(), 2);
        assert_eq!(syllabify_el("φαϊ").len(), 2);
    }

    #[test]
    fn test_merge_never() {
        assert_eq!(syllabify_el_mode("αστειάκιαν", Merge::Never).len(), 5);
    }

    #[test]
    fn test_merge_never_consonant_cluster() {
        assert_eq!(syllabify_el_mode("ενδο", Merge::Never).len(), 2);
    }

    #[test]
    fn test_merge_never_intersecting_diphthongs() {
        assert_eq!(syllabify_el_mode("αρουιν", Merge::Never).len(), 3);
        assert_eq!(syllabify_el_mode("αρουι", Merge::Never).len(), 3);
    }

    #[test]
    fn test_merge_never_extended() {
        assert_eq!(syllabify_el_mode("κίᾳ", Merge::Never).len(), 2);
        assert_eq!(syllabify_el_mode("ά̓κίᾳ", Merge::Never).len(), 3);
    }

    #[test]
    fn test_is_vowel_opt() {
        const VOWELS_LOWER: &str = "αειουωη";
        for code in 0x0370..=0x1FFF {
            if let Some(ch) = char::from_u32(code) {
                let base = base_lower(ch);
                if VOWELS_LOWER.contains(base) {
                    assert!(is_vowel_el_opt(ch));
                }
            }
        }
    }
}
