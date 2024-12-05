mod accents;
mod chars;
mod syllabify;
mod synizesis;

pub use accents::remove_accents;
pub use accents::to_mono;

pub use syllabify::syllabify_el;
pub use syllabify::syllabify_el_syn;
pub use syllabify::syllabify_gr;

// For testing
pub use syllabify::syllabify_gr_ref;
