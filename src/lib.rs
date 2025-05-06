#[macro_use]
pub mod macros;

mod accents;
mod chars;
mod monotonic;
mod syllabify;
mod synizesis;

pub mod constants;

pub use accents::Diacritic;
pub use accents::add_acute_at;
pub use accents::diacritic_pos;
pub use accents::has_any_diacritic;
pub use accents::has_diacritic;
pub use accents::has_diacritics;
pub use accents::remove_all_diacritics;
pub use accents::remove_diacritic_at;
pub use accents::remove_diacritics;

pub use chars::base_lower;
pub use chars::ends_with_diphthong;
pub use chars::is_greek_char;
pub use chars::is_greek_letter;
pub use chars::is_greek_word;

pub use monotonic::split_punctuation;
pub use monotonic::to_monotonic;

pub use syllabify::Merge;
pub use syllabify::Syllables;
pub use syllabify::syllabify;
pub use syllabify::syllabify_with_merge;
