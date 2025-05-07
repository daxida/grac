use std::ops::{Deref, DerefMut};

use crate::accents::has_diaeresis;
use crate::chars::base_lower;
use crate::synizesis::lookup_synizesis;

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

// For completion it contains:
// * archaic versions: άϊ, όϊ etc.
// * υι, even though this should be (probably!) always unmerged
#[rustfmt::skip]
const CANDIDATE_MERGING_DIPHTHONGS_EL: [(char, char); 19] = [
    // ai
    ('α', 'η'), ('ά', 'η'), ('α', 'ϊ'), ('α', 'ΐ'), ('ά', 'ι'), ('ά', 'ϊ'),
    // oi
    ('ο', 'η'), ('ό', 'η'), ('ο', 'ϊ'), ('ο', 'ΐ'), ('ό', 'ι'), ('ό', 'ϊ'),
    // ui (covers υι and ουι)
    ('υ', 'η'), ('ύ', 'η'), ('υ', 'ϊ'), ('υ', 'ΐ'), ('ύ', 'ι'), ('ύ', 'ϊ'),
    // Needs the extra υί to deal with ουί
    ('υ', 'ί'),
];

/// Locations to merge syllables at vowels.
///
/// # `Merge::Indices`
///
/// A slice of syllable indices where merging should occur.
/// They are 1-indexed counting from the end of the word.
///
/// In case of multiple indices, they should refer to the syllable positions
/// after the desired merging takes place (cf. syllabify documentation).
#[derive(Debug, Clone)]
pub enum Merge {
    Every,
    Never,
    // This is owned to simplify the python bindings
    Indices(Vec<usize>),
}

impl Merge {
    pub fn from_indices(indices: &[usize]) -> Self {
        Self::Indices(indices.to_vec())
    }

    fn to_bool(&self, idx_syllable: usize) -> bool {
        match self {
            Self::Every => true,
            Self::Never => false,
            Self::Indices(idxs) => idxs.contains(&idx_syllable),
        }
    }
}

/// Syllabify a modern Greek word.
///
/// Automatically detects synizesis.
///
/// # Example
///
/// ```
/// use grac::syllabify;
/// assert_eq!(syllabify("αρρώστια").join("-"), "αρ-ρώ-στια");
/// ```
#[allow(clippy::option_if_let_else)]
pub fn syllabify(s: &str) -> Syllables<'_> {
    match lookup_synizesis(s) {
        Some(res) => Syllables::from(res),
        _ => syllabify_impl(s, Merge::Never),
    }
}

/// Syllabify a modern Greek word.
///
/// # Example
///
/// ```
/// use grac::{syllabify_with_merge, Merge};
///
/// let word = "αστειάκια";
/// assert_eq!(syllabify_with_merge(word, Merge::Every).join("-"), "α-στειά-κια");
/// assert_eq!(syllabify_with_merge(word, Merge::Never).join("-"), "α-στει-ά-κι-α");
///
/// // Merge at the first syllable from the end.
/// let idxs = Merge::from_indices(&[1]);
/// assert_eq!(syllabify_with_merge(word, idxs).join("-"), "α-στει-ά-κια");
///
/// // Indices refer to syllable positions after each merge.
/// let idxs = Merge::from_indices(&[1, 2]);
/// assert_eq!(syllabify_with_merge(word, idxs).join("-"), "α-στειά-κια");
/// ```
pub fn syllabify_with_merge<'a>(s: &'a str, merge: Merge) -> Syllables<'a> {
    syllabify_impl(s, merge)
}

/// Return true if ch normalizes to a vowel (αοειηυω).
//
// Note that it can also return true when ch does not normalize to a vowel.
// mainly for characters we don't care about in the Greek unicode ranges (ex. Ͷ).
//
// The straightforward logic to match vowels is slower, even when sorted by frequency!
pub const fn is_vowel(ch: char) -> bool {
    match ch {
        '\u{0370}'..='\u{03FF}' => !is_consonant(ch),
        '\u{1F00}'..='\u{1FFF}' => !matches!(ch, 'ῤ' | 'ῥ' | 'Ῥ'),
        _ => false,
    }
}

#[rustfmt::skip]
const fn is_consonant(ch: char) -> bool {
    matches!(
        ch,
        // lowercase
        'β' | 'γ' | 'δ' | 'ζ' | 'θ' | 'κ' | 'λ' | 'μ' | 'ν' | 'ξ' |
        'π' | 'ρ' | 'σ' | 'ς' | 'τ' | 'φ' | 'χ' | 'ψ' |

        // uppercase
        'Β' | 'Γ' | 'Δ' | 'Ζ' | 'Θ' | 'Κ' | 'Λ' | 'Μ' | 'Ν' | 'Ξ' |
        'Π' | 'Ρ' | 'Σ' | 'Τ' | 'Φ' | 'Χ' | 'Ψ'
    )
}

pub fn is_diphthong(chs: &[char]) -> bool {
    match chs {
        [a, b] => {
            let pair = (base_lower(*a), base_lower(*b));
            DIPHTHONGS_EL.contains(&pair) && !has_diaeresis(*b)
        }
        _ => false,
    }
}

fn is_candidate_diphthong(chs: &[char]) -> bool {
    match chs {
        [a, b] => CANDIDATE_MERGING_DIPHTHONGS_EL.contains(&(*a, *b)),
        _ => false,
    }
}

fn is_consonant_cluster(a: char, b: char) -> bool {
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
    fn from_iter<T: IntoIterator<Item = S<'a>>>(iter: T) -> Self {
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
impl<'a> From<&[S<'a>]> for Syllables<'a> {
    fn from(slice: &[S<'a>]) -> Self {
        slice.iter().copied().collect()
    }
}

#[derive(Debug)]
enum State {
    Start,
    FoundVowel,
    FoundConsonant,
}

// Because we yield syllables in reverse order, we cannot refactor this into
// returning only an iterator (we need to allocate the syllables somehow).
fn syllabify_impl<'a>(s: &'a str, merge: Merge) -> Syllables<'a> {
    let mut out = Ty::with_capacity(8); // Found experimentally

    let mut state = State::Start;
    let mut idx_syllable = 1;
    let mut cur_merge = merge.to_bool(idx_syllable);

    // We'll walk backwards using char_indices().rev(), and buffer recent chars
    let mut to_byte = s.len();
    let mut buffer: [(usize, char); 3] = [(0, '\0'); 3]; // for peeking ahead

    macro_rules! dump_at {
        ($fr_byte:expr) => {{
            out.push(&s[$fr_byte..to_byte]);
            to_byte = $fr_byte;
            idx_syllable += 1;
            cur_merge = merge.to_bool(idx_syllable);
        }};
    }

    for (fr_byte, ch) in s.char_indices().rev() {
        // Slide buffer
        buffer.copy_within(0..2, 1);
        buffer[0] = (fr_byte, ch);

        let vowel = is_vowel(ch);

        // eprintln!(
        //     "* {:<15} {} {} {} {} \n| Buf {:?}",
        //     format!("{state:?}"),
        //     fr_byte,
        //     ch,
        //     vowel,
        //     out.join("-"),
        //     buffer
        // );

        match state {
            State::Start => {
                if vowel {
                    state = State::FoundVowel;
                }
            }
            State::FoundVowel => {
                if vowel {
                    let (next_idx, next_ch) = buffer[1];
                    let icd = is_candidate_diphthong(&[ch, next_ch]);
                    if cur_merge && (icd || matches!(ch, 'ι' | 'υ' | 'η' | 'ϊ')) {
                        if icd && !merge.to_bool(idx_syllable + 1) {
                            // όια
                            // dump the part after the iota
                            dump_at!(next_idx);
                        }
                        // keep advancing (=merge)
                    } else if !icd && is_diphthong(&[ch, next_ch]) {
                        let (after_next_idx, after_next_ch) = buffer[2];
                        if after_next_ch == 'ι' && to_byte > after_next_idx {
                            // ουι
                            // dump the part after the iota
                            dump_at!(after_next_idx);
                        }
                        // keep advancing (=merge)
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
                } else if !is_consonant_cluster(ch, next_ch) {
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
    fn test_is_diphthong() {
        assert!(is_diphthong(&['α', 'ι']));
        assert!(!is_diphthong(&['α', 'ε']));
        assert!(!is_diphthong(&['α', 'ϋ']));
    }

    #[test]
    fn test_syllabify() {
        assert_eq!(syllabify("μπεις").len(), 1);
        assert_eq!(syllabify("Ώστε").len(), 2);
    }

    #[test]
    fn test_syllabify_consonant_cluster() {
        assert_eq!(syllabify("στρες").len(), 1);
        assert_eq!(syllabify("ΣΤΡΕΣ").len(), 1);
        assert_eq!(syllabify("χλόη").len(), 2);
    }

    #[test]
    fn test_syllabify_diaeresis() {
        assert_eq!(syllabify("φαΐ").len(), 2);
        assert_eq!(syllabify("φαϊ").len(), 2);
    }

    #[test]
    fn test_merge_never() {
        assert_eq!(syllabify_with_merge("αστειάκιαν", Merge::Never).len(), 5);
    }

    #[test]
    fn test_merge_never_consonant_cluster() {
        assert_eq!(syllabify_with_merge("ενδο", Merge::Never).len(), 2);
    }

    #[test]
    fn test_merge_never_intersecting_diphthongs() {
        assert_eq!(syllabify_with_merge("αρουι", Merge::Never).len(), 3);
        assert_eq!(syllabify_with_merge("αρουιν", Merge::Never).len(), 3);
        assert_eq!(syllabify_with_merge("αλληλούια", Merge::Never).len(), 5);
        assert_eq!(syllabify_with_merge("παλαιικά", Merge::Never).len(), 4);
    }

    #[test]
    fn test_merge_never_extended() {
        assert_eq!(syllabify_with_merge("κίᾳ", Merge::Never).len(), 2);
        assert_eq!(syllabify_with_merge("ά̓κίᾳ", Merge::Never).len(), 3);
    }

    #[test]
    fn test_is_vowel_opt() {
        const VOWELS_LOWER: &str = "αειουωη";
        for code in 0x0370..=0x1FFF {
            if let Some(ch) = char::from_u32(code) {
                let base = base_lower(ch);
                if VOWELS_LOWER.contains(base) {
                    assert!(is_vowel(ch));
                }
            }
        }
    }
}
