mod accents;
mod chars;
mod monotonic;
mod syllabify;
mod synizesis;

pub use accents::add_acute;
pub use accents::remove_all_diacritics;
pub use accents::remove_diacritics;
pub use accents::Diacritic;

pub use monotonic::to_mono;

pub use syllabify::syllabify_el;
pub use syllabify::syllabify_el_mode;
pub use syllabify::syllabify_gr;

// For testing
pub use syllabify::syllabify_gr_ref;
