use grac::{is_diphthong, syllabify, syllabify_2, syllabify_3};
use paste::paste;

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
