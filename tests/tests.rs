use grac::{is_diphthong, syllabify, syllabify_2, syllabify_3};
use paste::paste;
use quickcheck::quickcheck;

#[test]
fn test_is_diphthong() {
    assert_eq!(is_diphthong("αι"), true);
    assert_eq!(is_diphthong("αε"), false);
    assert_eq!(is_diphthong("αϋ"), false);
}

macro_rules! mktest {
    ($input:expr, $expected:expr) => {
        paste! {
            #[test]
            fn [<syllabify_1_$input:lower>]() {
                let result = [<syllabify>]($input);
                assert_eq!(result, $expected);
            }
        }
        paste! {
            #[test]
            fn [<syllabify_2_$input:lower>]() {
                let result = [<syllabify_2>]($input);
                assert_eq!(result, $expected);
            }
        }
        paste! {
            #[test]
            fn [<syllabify_3_$input:lower>]() {
                let result = [<syllabify_3>]($input);
                assert_eq!(result, $expected);
            }
        }
    };
}

mktest!("γυναικός", vec!["γυ", "ναι", "κός"]);
mktest!("φῡ́ω", vec!["φῡ́", "ω"]);
mktest!("Μελέτες", vec!["Με", "λέ", "τες"]);

mktest!("Πυθαγόρας", vec!["Πυ", "θα", "γό", "ρας"]);
mktest!("Αλέξανδρος", vec!["Α", "λέ", "ξαν", "δρος"]);
mktest!("Ἀθήνα", vec!["Ἀ", "θή", "να"]);
mktest!("Ὅμηρος", vec!["Ὅ", "μη", "ρος"]);

mktest!("στρες", vec!["στρες"]);
mktest!("άνδρας", vec!["άν", "δρας"]);
mktest!("ἄρουι", vec!["ἄ", "ρου", "ι"]);
mktest!("ἄρουιν", vec!["ἄ", "ρου", "ιν"]);
// mktest!("ἄρουιν_", vec!["ἄ", "ρου", "ιν_"]);
mktest!("english", vec!["english"]);

mktest!("Ἀχαιιά", vec!["Ἀ", "χαι", "ι", "ά"]);
mktest!("Ἠελίοιο", vec!["Ἠ", "ε", "λί", "οι", "ο"]);
mktest!("Θρήικι", vec!["Θρή", "ι", "κι"]);

const GREEK_LETTERS: &[(u32, u32)] = &[
    (0x0370, 0x03FF), // Basic Greek and Coptic
    (0x1F00, 0x1FFF), // Greek Extended
];

#[derive(Debug, Clone)]
struct GreekWord(String);

impl quickcheck::Arbitrary for GreekWord {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let wlen = usize::arbitrary(g) % 200 + 1;
        let mut word = String::new();
        let letters: Vec<char> = GREEK_LETTERS
            .iter()
            .flat_map(|&(start, end)| (start..=end))
            .filter_map(char::from_u32)
            .collect();
        for _ in 0..wlen {
            let c = g.choose(&letters).unwrap();
            word.push(*c);
        }
        Self(word)
    }
}

quickcheck! {
    fn test_syllabify_equality(word: GreekWord) -> bool {
        let result_1 = syllabify(&word.0);
        let result_2 = syllabify_2(&word.0);
        let result_3 = syllabify_3(&word.0);
        result_1 == result_2 && result_2 == result_3
    }
}
