mod chars;

use chars::{base_lower, diaeresis, Accent, Breathing};

// By frequency: https://www.sttmedia.com/characterfrequency-greek
// const VOWELS: &str = "αοειηυω~";
const VOWELS: [char; 9] = [
    'α',
    'ο',
    'ε',
    'ι',
    'η',
    'υ',
    'ω',
    '~',
    Accent::Acute.as_char(),
];

fn is_vowel(ch: char) -> bool {
    VOWELS.contains(&base_lower(ch))
}

#[rustfmt::skip]
// We removed the trivial στρ from the original (since it used starts_with)
const VALID_CLUSTERS: [(char, char); 33] = [
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

fn is_valid_consonant_cluster(b: char, cs: &[char]) -> bool {
    let mut combined = vec![b];
    combined.extend_from_slice(cs);
    is_valid_consonant_cluster_chars(&combined)
}

fn is_valid_consonant_cluster_chars(chs: &[char]) -> bool {
    if let [a, b, ..] = chs {
        let pair = (base_lower(*a), base_lower(*b));
        return VALID_CLUSTERS.contains(&pair);
    }
    false
}

const VALID_DIPHTHONGS: [(char, char); 8] = [
    ('α', 'ι'),
    ('ε', 'ι'),
    ('ο', 'ι'),
    ('υ', 'ι'),
    ('α', 'υ'),
    ('ε', 'υ'),
    ('ο', 'υ'),
    ('η', 'υ'),
];

pub fn is_diphthong(s: &str) -> bool {
    let mut chars = s.chars();
    if let (Some(a), Some(b)) = (chars.next(), chars.next()) {
        is_diphthong_chars(&[a, b])
    } else {
        false
    }
}

pub fn is_diphthong_chars(chs: &[char]) -> bool {
    match chs {
        [a, b] => {
            let pair = (base_lower(*a), base_lower(*b));
            VALID_DIPHTHONGS.contains(&pair) && diaeresis(*b).is_none()
        }
        _ => false,
    }
}

pub fn syllabify(word: &str) -> Vec<String> {
    let mut syl = String::new();
    let mut result = Vec::new();
    let mut state = 0;

    for ch in word.chars().rev() {
        match state {
            0 => {
                syl.insert(0, ch);
                if is_vowel(ch) {
                    state = 1;
                }
            }
            1 => {
                if is_vowel(ch) || ch == Breathing::Rough.as_char() {
                    let mut it = syl.chars();
                    let fst = it.next().unwrap(); // We must have at least one char here

                    if fst == Accent::Acute.as_char() || fst == Breathing::Rough.as_char() {
                        syl.insert(0, ch);
                    } else if is_diphthong(format!("{}{}", ch, fst).as_str()) {
                        if it.next() == Some('ι') {
                            result.insert(0, syl.chars().skip(1).collect());
                            syl = format!("{}{}", ch, fst);
                        } else {
                            syl.insert(0, ch);
                        }
                    } else {
                        result.insert(0, syl.clone());
                        syl = ch.to_string();
                    }
                } else {
                    syl.insert(0, ch);
                    state = 2;
                }
            }
            2 => {
                if is_vowel(ch) {
                    result.insert(0, syl.clone());
                    syl = ch.to_string();
                    state = 1;
                } else if is_valid_consonant_cluster(ch, &syl.chars().collect::<Vec<_>>()) {
                    syl.insert(0, ch);
                } else {
                    result.insert(0, syl.clone());
                    syl = ch.to_string();
                    state = 0;
                }
            }
            _ => {}
        }
    }

    result.insert(0, syl.clone());
    result
}

#[inline(always)]
fn dump(chars: &[char], fr: usize, to: &mut usize, result: &mut Vec<String>) {
    result.push(chars[fr..*to].iter().collect::<String>());
}

#[inline(always)]
fn dumpmove(chars: &[char], fr: usize, to: &mut usize, result: &mut Vec<String>) {
    dump(chars, fr, to, result);
    *to = fr;
}

pub fn syllabify_2(word: &str) -> Vec<String> {
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
                    } else if is_diphthong(&chars[fr..fr + 2].iter().collect::<String>()) {
                        // Two consecutive overlapping diphthongs?
                        if chars.get(fr + 2) == Some(&'ι') {
                            // Dump only the part after the iota
                            if fr + 2 < to {
                                dump(&chars, fr + 2, &mut to, &mut result);
                                to = fr + 2;
                            }
                        }
                    } else {
                        dumpmove(&chars, fr + 1, &mut to, &mut result);
                    }
                } else {
                    state = 2;
                }
            }
            2 => {
                if is_vowel(ch) {
                    dumpmove(&chars, fr + 1, &mut to, &mut result);
                    state = 1;
                } else if !is_valid_consonant_cluster_chars(&chars[fr..to]) {
                    dumpmove(&chars, fr + 1, &mut to, &mut result);
                    state = 0;
                }
            }
            _ => {}
        }
    }

    if 0 < to {
        dump(&chars, 0, &mut to, &mut result);
    }

    result.reverse();
    result
}

pub fn syllabify_3(word: &str) -> Vec<String> {
    let chars: Vec<char> = word.chars().collect();
    let mut pos = chars.len();
    let mut result = Vec::new();

    while let Some(syllable) = parse_syllable(&chars, &mut pos) {
        result.push(syllable);
    }

    result.reverse();
    result
}

fn parse_syllable(chars: &[char], pos: &mut usize) -> Option<String> {
    let to = *pos;

    move_coda(chars, pos);
    move_nucleus(chars, pos);
    move_onset(chars, pos);

    let syllable = chars[*pos..to].iter().collect::<String>();

    if !syllable.is_empty() {
        Some(syllable)
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
        if to - *pos > 0 {
            if is_diphthong_chars(&chars[*pos - 1..*pos + 1]) {
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
        // Invalid consonant cluster
        && !(to - *pos > 0 && !is_valid_consonant_cluster_chars(&chars[*pos - 1..to]))
    {
        *pos -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_syllable() {
        let chars: Vec<char> = "στρες".chars().collect();
        let mut pos = chars.len();

        let syllable = parse_syllable(&chars, &mut pos);
        assert_eq!(syllable, Some("στρες".to_string()));
    }

    #[test]
    fn test_syllabify_pythagoras() {
        let result = syllabify_3("Πυθαγόρας");
        assert_eq!(result, vec!["Πυ", "θα", "γό", "ρας"]);
    }

    #[test]
    fn test_syllabify_alexander_2() {
        let result = syllabify_2("Αλέξανδρος");
        assert_eq!(result, vec!["Α", "λέ", "ξαν", "δρος"]);
    }

    #[test]
    fn test_syllabify_stress() {
        let result = syllabify_2("στρες");
        assert_eq!(result, vec!["στρες"]);
    }

    #[test]
    fn test_syllabify_andras() {
        let result = syllabify_2("άνδρας");
        assert_eq!(result, vec!["άν", "δρας"]);
    }

    #[test]
    fn test_syllabify_alexander() {
        let result = syllabify_3("Αλέξανδρος");
        assert_eq!(result, vec!["Α", "λέ", "ξαν", "δρος"]);
    }

    #[test]
    fn test_syllabify_athens() {
        let result = syllabify_3("Ἀθήνα");
        assert_eq!(result, vec!["Ἀ", "θή", "να"]);
    }

    #[test]
    fn test_syllabify_homer() {
        let result = syllabify_3("Ὅμηρος");
        assert_eq!(result, vec!["Ὅ", "μη", "ρος"]);
    }

    #[test]
    fn test_syllabify_aroui() {
        let result = syllabify_3("ἄρουι");
        assert_eq!(result, vec!["ἄ", "ρου", "ι"]);
    }

    #[test]
    fn test_syllabify2_arouin() {
        let result = syllabify_3("ἄρουιν");
        assert_eq!(result, vec!["ἄ", "ρου", "ιν"]);
    }
}
