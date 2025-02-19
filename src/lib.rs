mod accents;
mod chars;
mod monotonic;
mod syllabify;
mod synizesis;

pub mod constants;

pub use accents::add_acute_at;
pub use accents::diacritic_pos;
pub use accents::has_any_diacritic;
pub use accents::has_diacritic;
pub use accents::has_diacritics;
pub use accents::remove_all_diacritics;
pub use accents::remove_diacritic_at;
pub use accents::remove_diacritics;
pub use accents::Diacritic;

pub use chars::base_lower;
pub use chars::ends_with_diphthong;
pub use chars::is_greek_char;
pub use chars::is_greek_word;

pub use monotonic::split_word_punctuation;
pub use monotonic::to_mono;

pub use syllabify::is_vowel_el;
pub use syllabify::syllabify_el;
pub use syllabify::syllabify_el_mode;
pub use syllabify::syllabify_gr;
pub use syllabify::Synizesis;

// For testing
pub use syllabify::syllabify_gr_ref;
