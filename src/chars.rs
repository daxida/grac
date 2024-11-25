use unicode_normalization::char::decompose_canonical;

// Doing this instead of ranges allows for const, but does not account for char gaps.
const fn is_modern_lower_greek(ch: char) -> bool {
    ch >= '\u{03B1}' && ch <= '\u{03C9}'
}

// Faster than an .nfd() call for a single char.
pub fn base(ch: char) -> char {
    let mut base_char = None;
    decompose_canonical(ch, |c| {
        base_char.get_or_insert(c);
    });
    base_char.unwrap_or(ch)
}

pub fn base_lower(ch: char) -> char {
    if is_modern_lower_greek(ch) {
        // Cheap function
        ch
    } else {
        // Expensive function
        base(ch).to_lowercase().next().unwrap_or(ch)
    }
}
