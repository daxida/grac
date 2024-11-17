use grac::{is_diphthong, syllabify, syllabify_2, syllabify_3};

#[test]
fn test_is_diphthong() {
    assert_eq!(is_diphthong("αι"), true);
    assert_eq!(is_diphthong("αε"), false);
    assert_eq!(is_diphthong("αϋ"), false);
}

#[test]
fn test_syllabify_gynaikos() {
    let result = syllabify("γυναικός");
    assert_eq!(result, vec!["γυ", "ναι", "κός"]);
}

#[test]
fn test_syllabify_fyō() {
    let result = syllabify("φῡ́ω");
    assert_eq!(result, vec!["φῡ́", "ω"]);
}

#[test]
fn test_syllabify_meletes() {
    let result = syllabify("Μελέτες");
    assert_eq!(result, vec!["Με", "λέ", "τες"]);
}

#[test]
fn test_syllabify2_meletes() {
    let result = syllabify_2("Μελέτες");
    assert_eq!(result, vec!["Με", "λέ", "τες"]);
}

#[test]
fn test_syllabify2_gynaikos() {
    let result = syllabify_2("γυναικός");
    assert_eq!(result, vec!["γυ", "ναι", "κός"]);
}

//////////////////////////////////

#[test]
fn test_syllabify_pythagoras() {
    let result = syllabify("Πυθαγόρας");
    assert_eq!(result, vec!["Πυ", "θα", "γό", "ρας"]);
}

#[test]
fn test_syllabify_alexander() {
    let result = syllabify("Αλέξανδρος");
    assert_eq!(result, vec!["Α", "λέ", "ξαν", "δρος"]);
}

#[test]
fn test_syllabify_athens() {
    let result = syllabify("Ἀθήνα");
    assert_eq!(result, vec!["Ἀ", "θή", "να"]);
}

#[test]
fn test_syllabify_homer() {
    let result = syllabify("Ὅμηρος");
    assert_eq!(result, vec!["Ὅ", "μη", "ρος"]);
}

#[test]
fn test_syllabify2_pythagoras() {
    let result = syllabify_2("Πυθαγόρας");
    assert_eq!(result, vec!["Πυ", "θα", "γό", "ρας"]);
}

#[test]
fn test_syllabify2_alexander() {
    let result = syllabify_2("Αλέξανδρος");
    assert_eq!(result, vec!["Α", "λέ", "ξαν", "δρος"]);
}

#[test]
fn test_syllabify2_athens() {
    let result = syllabify_2("Ἀθήνα");
    assert_eq!(result, vec!["Ἀ", "θή", "να"]);
}

#[test]
fn test_syllabify2_homer() {
    let result = syllabify_2("Ὅμηρος");
    assert_eq!(result, vec!["Ὅ", "μη", "ρος"]);
}

/////////////////////////////////

#[test]
fn test_syllabify2_aroui() {
    let result = syllabify_2("ἄρουι");
    assert_eq!(result, vec!["ἄ", "ρου", "ι"]);
}

#[test]
fn test_syllabify_aroui() {
    let result = syllabify("ἄρουι");
    assert_eq!(result, vec!["ἄ", "ρου", "ι"]);
}

#[test]
fn test_syllabify2_arouin() {
    let result = syllabify_2("ἄρουιν");
    assert_eq!(result, vec!["ἄ", "ρου", "ιν"]);
}

#[test]
fn test_syllabify_arouin() {
    let result = syllabify("ἄρουιν");
    assert_eq!(result, vec!["ἄ", "ρου", "ιν"]);
}

#[test]
fn test_syllabify_axa() {
    let result = syllabify("Ἀχαιιά");
    assert_eq!(result, vec!["Ἀ", "χαι", "ι", "ά"]);
}

#[test]
fn test_syllabify_2_axa() {
    let result = syllabify_2("Ἀχαιιά");
    assert_eq!(result, vec!["Ἀ", "χαι", "ι", "ά"]);
}

#[test]
fn test_syllabify_helioio() {
    // This may fail if we try to be too smart at "is_vowel"
    let result = syllabify_2("Ἠελίοιο");
    assert_eq!(result, vec!["Ἠ", "ε", "λί", "οι", "ο"])
}

#[test]
fn test_syllabify_thriiki() {
    // This may fail if we try to be too smart at "base_lower"
    let result = syllabify("Θρήικι");
    assert_eq!(result, vec!["Θρή", "ι", "κι"])
}

#[test]
fn test_syllabify_thriiki_2() {
    // This may fail if we try to be too smart at "base_lower"
    let result = syllabify_2("Θρήικι");
    assert_eq!(result, vec!["Θρή", "ι", "κι"])
}

#[test]
fn test_syllabify_thriiki_3() {
    // This may fail if we try to be too smart at "base_lower"
    let result = syllabify_3("Θρήικι");
    assert_eq!(result, vec!["Θρή", "ι", "κι"])
}
