use crate::accents::{diaeresis, Accent, Breathing};
use crate::chars::base_lower;

// By frequency: https://www.sttmedia.com/characterfrequency-greek
#[rustfmt::skip]
const VOWELS_GR: [char; 9] = [
    'α', 'ο', 'ε', 'ι', 'η', 'υ', 'ω',
    '~', Accent::ACUTE,
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

// TODO: Sort me
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

// NOTE: This is likely slower than just duplicating the code for both languages.
pub fn syllabify_gr(word: &str) -> Vec<&str> {
    // syllabify_gr_(word)
    syllabify_lang(word, &GR)
}

pub fn syllabify_el(word: &str) -> Vec<&str> {
    syllabify_lang(word, &EL)
}

fn is_vowel(ch: char, lang: &Lang) -> bool {
    lang.vowels.contains(&base_lower(ch))
}

fn is_diphthong(chs: &[char], lang: &Lang) -> bool {
    match chs {
        [a, b] => {
            let pair = (base_lower(*a), base_lower(*b));
            lang.diphthongs.contains(&pair) && diaeresis(*b).is_none()
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

fn syllabify_lang<'a>(word: &'a str, lang: &Lang) -> Vec<&'a str> {
    let chars: Vec<char> = word.chars().collect();
    let mut pos = chars.len();
    let mut result = Vec::new();

    while let Some(syllable) = parse_syllable(word, &chars, &mut pos, lang) {
        result.push(syllable);
    }

    result.reverse();
    result
}

fn parse_syllable<'a>(
    word: &'a str,
    chars: &[char],
    pos: &mut usize,
    lang: &Lang,
) -> Option<&'a str> {
    let to = *pos;

    move_coda(chars, pos, lang);
    move_nucleus(chars, pos, lang);
    move_onset(chars, pos, lang);

    if *pos < to {
        let fr_byte = get_byte_offset(*pos, chars);
        let to_byte = get_byte_offset(to, chars);
        Some(&word[fr_byte..to_byte])
    } else {
        None
    }
}

fn move_coda(chars: &[char], pos: &mut usize, lang: &Lang) {
    while *pos > 0 && !is_vowel(chars[*pos - 1], lang) {
        *pos -= 1;
    }
}

fn move_nucleus(chars: &[char], pos: &mut usize, lang: &Lang) {
    let to = *pos;
    while *pos > 0 && (is_vowel(chars[*pos - 1], lang) || chars[*pos - 1] == Breathing::ROUGH) {
        if to - *pos > 0 && chars[*pos] != Accent::ACUTE && chars[*pos] != Breathing::ROUGH {
            if is_diphthong(&chars[*pos - 1..*pos + 1], lang) {
                if to - *pos > 1 && chars.get(*pos + 1) == Some(&'ι') {
                    *pos += 1;
                    break;
                }
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

// fn syllabify_gr_(word: &str) -> Vec<&str> {
//     let chars: Vec<char> = word.chars().collect();
//     let mut pos = chars.len();
//     let mut result = Vec::new();
//
//     while let Some(syllable) = parse_syllable_(word, &chars, &mut pos) {
//         result.push(syllable);
//     }
//
//     result.reverse();
//     result
// }
//
// fn parse_syllable_<'a>(word: &'a str, chars: &[char], pos: &mut usize) -> Option<&'a str> {
//     let to = *pos;
//
//     move_coda_(chars, pos);
//     move_nucleus_(chars, pos);
//     move_onset_(chars, pos);
//
//     if *pos < to {
//         let fr_byte = chars[..*pos].iter().map(|c| c.len_utf8()).sum::<usize>();
//         let to_byte = chars[..to].iter().map(|c| c.len_utf8()).sum::<usize>();
//         Some(&word[fr_byte..to_byte])
//     } else {
//         None
//     }
// }
//
// fn move_coda_(chars: &[char], pos: &mut usize) {
//     while *pos > 0 && !is_vowel_gr(chars[*pos - 1]) {
//         *pos -= 1;
//     }
// }
//
// fn move_nucleus_(chars: &[char], pos: &mut usize) {
//     let to = *pos;
//     while *pos > 0 && (is_vowel_gr(chars[*pos - 1]) || chars[*pos - 1] == Breathing::ROUGH) {
//         if to - *pos > 0 && chars[*pos] != Accent::ACUTE && chars[*pos] != Breathing::ROUGH {
//             if is_diphthong_gr(&chars[*pos - 1..*pos + 1]) {
//                 if to - *pos > 1 && chars.get(*pos + 1) == Some(&'ι') {
//                     *pos += 1;
//                     break;
//                 }
//             } else {
//                 break;
//             }
//         }
//         *pos -= 1;
//     }
// }
//
// fn move_onset_(chars: &[char], pos: &mut usize) {
//     let to = *pos;
//     while *pos > 0
//         && !is_vowel_gr(chars[*pos - 1])
//         // If we reach a consonant cluster we keep moving
//         && (to == *pos || is_consonant_cluster_gr(&chars[*pos - 1..to] ))
//     {
//         *pos -= 1;
//     }
// }

///////////// Oracle reference. Not intended for use.

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
                if is_vowel_gr(ch) || ch == Breathing::ROUGH {
                    let prev = chars[fr + 1];

                    if prev == Accent::ACUTE || prev == Breathing::ROUGH {
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
        let mut pos = chars.len();

        let syllable = parse_syllable(word, &chars, &mut pos, &GR);
        assert_eq!(syllable, Some("στρες"));
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
}
