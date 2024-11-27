use grac::{syllabify_el, syllabify_gr, syllabify_gr_ref};
use quickcheck::quickcheck;

macro_rules! mktests_gr {
    ($group_name:ident, $([$input:expr, $expected:expr]),* $(,)?) => {
        #[test]
        fn $group_name() {
            let test_cases = vec![
                $(
                    ($input, $expected),
                )*
            ];

            for (input, expected) in test_cases {
                let result = syllabify_gr(input);
                let tc_expected = expected.split('-').collect::<Vec<_>>();
                assert_eq!(result, tc_expected);

                let result = syllabify_gr_ref(input);
                let tc_expected = expected.split('-').collect::<Vec<_>>();
                assert_eq!(result, tc_expected);
            }
        }
    };
}

macro_rules! mktests_el {
    ($group_name:ident, $([$input:expr, $expected:expr]),* $(,)?) => {
        #[test]
        fn $group_name() {
            let test_cases = vec![
                $(
                    ($input, $expected),
                )*
            ];

            for (input, expected) in test_cases {
                let result = syllabify_el(input);
                let tc_expected = expected.split('-').collect::<Vec<_>>();
                assert_eq!(result, tc_expected);
            }
        }
    };
}

mktests_gr!(
    syllabify_gr_basic,
    ["γυναικός", "γυ-ναι-κός"],
    ["φῡ́ω", "φῡ́-ω"],
    ["Μελέτες", "Με-λέ-τες"],
    ["στρες", "στρες"],
    ["άνδρας", "άν-δρας"],
    ["ἄρουι", "ἄ-ρου-ι"],
    ["ἄρουιν", "ἄ-ρου-ιν"],
    ["Ἀχαιιά", "Ἀ-χαι-ι-ά"],
    ["Ἠελίοιο", "Ἠ-ε-λί-οι-ο"],
    ["Θρήικι", "Θρή-ι-κι"],
    ["Ἠοῖα", "Ἠ-οῖ-α"],
    ["κόσμος", "κό-σμος"],
);

mktests_gr!(
    syllabify_gr_names,
    ["Πυθαγόρας", "Πυ-θα-γό-ρας"],
    ["Αλέξανδρος", "Α-λέ-ξαν-δρος"],
    ["Ἀθήνα", "Ἀ-θή-να"],
    ["Ὅμηρος", "Ὅ-μη-ρος"],
);

mktests_el!(
    syllabify_el_basic,
    ["ἄρουιν", "ἄ-ρου-ιν"],
    ["οιωνός", "οι-ω-νός"],
);

mktests_el!(
    syllabify_el_double_cons_same,
    ["μέλισσα", "μέ-λισ-σα"],
    ["θάλασσα", "θά-λασ-σα"],
    ["Ελλάδα", "Ελ-λά-δα"],
);

mktests_el!(
    syllabify_el_double_cons,
    ["κόσμος", "κό-σμος"],
    ["δεσμός", "δε-σμός"],
    ["πάντα", "πά-ντα"],
    ["Τζιτζίκι", "Τζι-τζί-κι"],
    // ["βούρτσα", "βούρ-τσα"],
    // ["γκολτζής", "γκολ-τζής"],
    // ["γλεντζές", "γλε-ντζές"],
    ["τμήμα", "τμή-μα"],
    // χν
    ["χνούδι", "χνού-δι"],
    ["αχνός", "α-χνός"],
);

mktests_el!(
    syllabify_el_triple_cons,
    ["εχθρός", "ε-χθρός"],
    ["άσπρος", "ά-σπρος"],
    ["αντλώ", "α-ντλώ"],
);

// Diaeresis
mktests_el!(
    syllabify_el_diaeresis,
    ["Αγλαΐα", "Α-γλα-ΐ-α"],
    ["αδενοϋπόφυση", "α-δε-νο-ϋ-πό-φυ-ση"]
);

// Depends on the speaker. This can not be a general rule.
// https://teachergeorgiasclass.weebly.com/uploads/4/5/0/7/45072177/Κάτω_απ_το_χιόνι_2.pdf
// mktests_el!(
//     syllabify_el_gorgias,
//     ["αηδόνι", "αη-δό-νι"],
//     ["δουλειά", "δου-λειά"],
//     ["κάποια", "κά-ποια"],
// );

// http://ebooks.edu.gr/ebooks/v/html/8547/2009/Grammatiki_E-ST-Dimotikou_html-apli/index_B4a.html
// https://melobytes.gr/el/app/syllavismos
mktests_el!(
    syllabify_el_dimotiko,
    // Rule 1: A consonant between two vowels is grouped with the second vowel.
    ["έχω", "έ-χω"],
    // Rule 2: Diphthongs, vowel combinations like αυ, ευ, ου, and diphthongs remain unbroken.
    ["ουρανός", "ου-ρα-νός"],
    ["γάιδαρος", "γάι-δα-ρος"],
    ["μπέικον", "μπέι-κον"],
    ["άυλος", "άυ-λος"],
    ["κορόιδο", "κο-ρόι-δο"],
    ["ναύτης", "ναύ-της"],
    // Rule 3: Two consonants between vowels are grouped if they start a valid Greek word, else split.
    ["ατμός", "α-τμός"],      // τμ -> valid start of a word
    ["έρχομαι", "έρ-χο-μαι"], // ρχ -> invalid start of a word
    // Rule 4: Three or more consonants between vowels are grouped if the first two start a valid Greek word.
    ["αστράφτω", "α-στρά-φτω"], // στ -> valid start of a word
    // ["σφυρίχτρα", "σφυ-ρί-χτρα"], // χτ -> valid start of a word
    ["άνθρωπος", "άν-θρω-πος"], // νθρ -> no valid word starts with νθρ
    // Rule 5: Identical consonants split, first goes with the preceding vowel, second with the following vowel.
    ["φεγγάρι", "φεγ-γά-ρι"],
    ["σύννεφο", "σύν-νε-φο"],
);

mktests_el!(
    syllabify_el_sivas_grammar,
    // Rule 3: Three or more consonants between vowels
    ["αισχός", "αι-σχός"],
    ["εκστρατεία", "εκ-στρα-τεί-α"],
    // Differs with hyphenation
    ["σκαντζόχοιροι", "σκα-ντζό-χοι-ροι"],
    // Rule 4:
    ["μπουμπουκάκι", "μπου-μπου-κά-κι"],
    ["αμπέλι", "α-μπέ-λι"],
    ["νταντά", "ντα-ντά"],
    ["πέντε", "πέ-ντε"],
    ["μπαγκέτα", "μπα-γκέ-τα"],
    ["μουγκρίζω", "μου-γκρί-ζω"],
);

// https://github.com/datio/grhyph/blob/master/grhyph_test.go
mktests_el!(
    syllabify_el_grhyph,
    ["άκαμπτος", "ά-κα-μπτος"],
    ["άλμπατρος", "άλ-μπα-τρος"],
    ["έκθλιψη", "έκ-θλι-ψη"],
    ["έκπληκτος", "έκ-πλη-κτος"],
    ["έμπνευση", "έ-μπνευ-ση"],
    ["ένσφαιρος", "έν-σφαι-ρος"],
    ["ίντσα", "ί-ντσα"],
    ["αεροελεγκτής", "α-ε-ρο-ε-λε-γκτής"],
    ["αισχρολόγος", "αι-σχρο-λό-γος"],
    ["αλτρουισμός", "αλ-τρου-ι-σμός"],
    ["αμφιβληστροειδής", "αμ-φι-βλη-στρο-ει-δής"],
    ["ανεξάντλητος", "α-νε-ξά-ντλη-τος"],
    ["ανυπέρβλητος", "α-νυ-πέρ-βλη-τος"],
    ["αργκό", "αρ-γκό"],
    ["αρθρογραφία", "αρ-θρο-γρα-φί-α"],
    ["ασύγγνωστος", "α-σύγ-γνω-στος"],
    ["βολφράμιο", "βολ-φρά-μι-ο"],
    ["βούρτσα", "βούρ-τσα"],
    ["γκολτζής", "γκολ-τζής"],
    ["γλεντζές", "γλε-ντζές"],
    ["Δεκέμβριος", "Δε-κέμ-βρι-ος"],
    ["διόπτρα", "δι-ό-πτρα"],
    ["εγγλέζικος", "εγ-γλέ-ζι-κος"],
    ["εισπλέω", "ει-σπλέ-ω"],
    ["εισπνοή", "ει-σπνο-ή"],
    ["εισπράκτορας", "ει-σπρά-κτο-ρας"],
    ["εκδρομέας", "εκ-δρο-μέ-ας"],
    ["εκδρομή", "εκ-δρο-μή"],
    ["εκθρόνιση", "εκ-θρό-νι-ση"],
    ["εκκρεμότητα", "εκ-κρε-μό-τη-τα"],
    ["εκπνοή", "εκ-πνο-ή"],
    ["εκπρόσωπος", "εκ-πρό-σω-πος"],
    ["εκπτωτικός", "εκ-πτω-τι-κός"],
    ["εκστομίζω", "εκ-στο-μί-ζω"],
    ["εκσφενδονισμός", "εκ-σφεν-δο-νι-σμός"],
    ["εκτρέφω", "ε-κτρέ-φω"],
    ["εκφραστικός", "εκ-φρα-στι-κός"],
    ["ελκτικός", "ελ-κτι-κός"],
    ["εμβληματικός", "εμ-βλη-μα-τι-κός"],
    ["ενθρόνιση", "εν-θρό-νι-ση"],
    ["ευστροφία", "ευ-στρο-φί-α"],
    ["εχθροπραξία", "ε-χθρο-πρα-ξί-α"],
    ["ινκόγκνιτο", "ιν-κό-γκνι-το"],
    ["ινστιτούτο", "ιν-στι-τού-το"],
    ["ισχνότητα", "ι-σχνό-τη-τα"],
    ["καλντερίμι", "καλ-ντε-ρί-μι"],
    ["καμτσίκι", "καμ-τσί-κι"],
    ["καρτποστάλ", "καρτ-πο-στάλ"],
    ["κομπλιμέντο", "κο-μπλι-μέ-ντο"],
    ["κύλινδρος", "κύ-λιν-δρος"],
    ["μετεγγραφή", "με-τεγ-γρα-φή"],
    ["μπαχτσές", "μπα-χτσές"],
    ["νομενκλατούρα", "νο-μεν-κλα-τού-ρα"],
    ["νταρντάνα", "νταρ-ντά-να"],
    ["ντόμπρος", "ντό-μπρος"],
    ["πάμφθηνα", "πάμ-φθη-να"],
    ["πανσπερμία", "παν-σπερ-μί-α"],
    ["παρεκκλήσι", "πα-ρεκ-κλή-σι"],
    ["πορθμός", "πορθ-μός"],
    ["προσβλέπω", "προ-σβλέ-πω"],
    ["πρόσκληση", "πρό-σκλη-ση"],
    ["πρόσκρουση", "πρό-σκρου-ση"],
    ["πρόσκτηση", "πρό-σκτη-ση"],
    ["πρόσπτωση", "πρό-σπτω-ση"],
    ["ράφτρα", "ρά-φτρα"],
    ["ροσμπίφ", "ρο-σμπίφ"],
    ["σάλτσα", "σάλ-τσα"],
    ["σεντράρισμα", "σε-ντρά-ρι-σμα"],
    ["στιλπνός", "στιλ-πνός"],
    ["συγκλονιστικός", "συ-γκλο-νι-στι-κός"],
    ["σφυρίχτρα", "σφυ-ρί-χτρα"],
    ["σύμπτωση", "σύ-μπτω-ση"],
    ["σύντμηση", "σύ-ντμη-ση"],
    ["τερπνότητα", "τερ-πνό-τη-τα"],
    ["τζαμτζής", "τζαμ-τζής"],
    ["Τουρκμενιστάν", "Τουρκ-με-νι-στάν"],
    ["τουρμπίνα", "τουρ-μπί-να"],
    ["τροτσκισμός", "τρο-τσκι-σμός"],
    ["τσουγκράνα", "τσου-γκρά-να"],
    ["υπαρκτός", "υ-παρ-κτός"],
    ["υπερδραστήριος", "υ-περ-δρα-στή-ρι-ος"],
    ["υπερκράτος", "υ-περ-κρά-τος"],
    ["υπερπλήρης", "υ-περ-πλή-ρης"],
    ["υπερσκελίζω", "υ-περ-σκε-λί-ζω"],
    ["υπερσταθμός", "υ-περ-σταθ-μός"],
    ["υπερσύγχρονος", "υ-περ-σύγ-χρο-νος"],
    ["υπερτραφής", "υ-περ-τρα-φής"],
    ["υπερχρονίζω", "υ-περ-χρο-νί-ζω"],
    ["φλαμίνγκο", "φλα-μίν-γκο"],
    ["φολκλορισμός", "φολ-κλο-ρι-σμός"],
);

// Synizesis
// mktests_el!(
//     ["αστέρια", vec!["α", "στέ", "ρια"]],
//     ["αστειάκια", vec!["α", "στει", "ά", "κια"]],
// );

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
        let result_1 = syllabify_gr_ref(&word.0);
        let result_2 = syllabify_gr(&word.0);
        result_1 == result_2
    }
}
