mod accents;
mod chars;
mod syllabify;

pub use accents::remove_accents;
pub use accents::to_mono;

pub use accents::Accent;
pub use accents::Breathing;

pub use syllabify::syllabify_el;
pub use syllabify::syllabify_gr;

// For testing
pub use syllabify::syllabify_gr_ref;
