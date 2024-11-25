use crate::accents::{diaeresis, Accent, Breathing};
use crate::chars::base_lower;

// By frequency: https://www.sttmedia.com/characterfrequency-greek
#[rustfmt::skip]
const VOWELS: [char; 9] = [
    'α', 'ο', 'ε', 'ι', 'η', 'υ', 'ω',
    '~', Accent::Acute.as_char(),
];

#[rustfmt::skip]
const CONS_CLUSTERS: [(char, char); 33] = [
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

const DIPHTHONGS: [(char, char); 8] = [
    ('α', 'ι'),
    ('ε', 'ι'),
    ('ο', 'ι'),
    ('υ', 'ι'),
    ('α', 'υ'),
    ('ε', 'υ'),
    ('ο', 'υ'),
    ('η', 'υ'),
];

fn is_vowel(ch: char) -> bool {
    VOWELS.contains(&base_lower(ch))
}

fn is_consonant_cluster(chs: &[char]) -> bool {
    match chs {
        [a, b, ..] => {
            let pair = (base_lower(*a), base_lower(*b));
            CONS_CLUSTERS.contains(&pair)
        }
        _ => false,
    }
}

fn is_diphthong(chs: &[char]) -> bool {
    match chs {
        [a, b] => {
            let pair = (base_lower(*a), base_lower(*b));
            DIPHTHONGS.contains(&pair) && diaeresis(*b).is_none()
        }
        _ => false,
    }
}

pub fn syllabify(word: &str) -> Vec<&str> {
    let chars: Vec<char> = word.chars().collect();
    let mut pos = chars.len();
    let mut result = Vec::new();

    while let Some(syllable) = parse_syllable(word, &chars, &mut pos) {
        result.push(syllable);
    }

    result.reverse();
    result
}

fn parse_syllable<'a>(word: &'a str, chars: &[char], pos: &mut usize) -> Option<&'a str> {
    let to = *pos;

    move_coda(chars, pos);
    move_nucleus(chars, pos);
    move_onset(chars, pos);

    if *pos < to {
        let fr_byte = chars[..*pos].iter().map(|c| c.len_utf8()).sum::<usize>();
        let to_byte = chars[..to].iter().map(|c| c.len_utf8()).sum::<usize>();
        Some(&word[fr_byte..to_byte])
    } else {
        None
    }
}

fn move_coda(chars: &[char], pos: &mut usize) {
    while *pos > 0 && !is_vowel(chars[*pos - 1]) {
        *pos -= 1;
    }
}

fn move_nucleus(chars: &[char], pos: &mut usize) {
    let to = *pos;
    while *pos > 0 && (is_vowel(chars[*pos - 1]) || chars[*pos - 1] == Breathing::Rough.as_char()) {
        if to - *pos > 0
            && chars[*pos] != Accent::Acute.as_char()
            && chars[*pos] != Breathing::Rough.as_char()
        {
            if is_diphthong(&chars[*pos - 1..*pos + 1]) {
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

fn move_onset(chars: &[char], pos: &mut usize) {
    let to = *pos;
    while *pos > 0
        && !is_vowel(chars[*pos - 1])
        && (to == *pos || is_consonant_cluster(&chars[*pos - 1..to]))
    {
        *pos -= 1;
    }
}

///////////// Oracle reference. Not intended for use.

#[inline(always)]
fn dump<'a>(
    chars: &[char],
    fr: usize,
    to: &mut usize,
    result: &mut Vec<&'a str>,
    original: &'a str,
) {
    let start = chars[..fr].iter().map(|c| c.len_utf8()).sum::<usize>();
    let end = chars[..*to].iter().map(|c| c.len_utf8()).sum::<usize>();
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

pub fn syllabify_ref(word: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut state = 0;
    let chars: Vec<char> = word.chars().collect();
    let mut to = chars.len();

    for (fr, &ch) in chars.iter().enumerate().rev() {
        match state {
            0 if is_vowel(ch) => state = 1,
            1 => {
                if is_vowel(ch) || ch == Breathing::Rough.as_char() {
                    let prev = chars[fr + 1];

                    if prev == Accent::Acute.as_char() || prev == Breathing::Rough.as_char() {
                        // Do nothing
                    } else if is_diphthong(&chars[fr..fr + 2]) {
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
                if is_vowel(ch) {
                    dumpmove(&chars, fr + 1, &mut to, &mut result, word);
                    state = 1;
                } else if !is_consonant_cluster(&chars[fr..to]) {
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

        let syllable = parse_syllable(word, &chars, &mut pos);
        assert_eq!(syllable, Some("στρες"));
    }

    #[test]
    fn test_is_diphthong() {
        assert!(is_diphthong(&['α', 'ι']));
        assert!(!is_diphthong(&['ι', 'α', 'ι']));
        assert!(!is_diphthong(&['α', 'ε']));
        assert!(!is_diphthong(&['α', 'ϋ']));
    }
}
