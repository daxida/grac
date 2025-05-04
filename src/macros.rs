//! Utility macros.
//!
//! Cf. <https://users.rust-lang.org/t/expanding-const-str-arrays/126374/6>

/// Expand the cartesian product of two const &str arrays.
///
/// # Example
///
/// ```
/// use grac::expand;
///
/// const LEMMAS: [&str; 2] = ["νέ", "νι"];
/// const ENDINGS: [&str; 3] = ["ος", "ο", "ου"];
/// const EXPANDED: [&str; 6] = expand!(LEMMAS, ENDINGS);
/// assert_eq!(EXPANDED, ["νέος", "νέο", "νέου", "νιος", "νιο", "νιου"])
/// ```
#[macro_export]
macro_rules! expand {
    ($l:expr, $r:expr $(,)?) => {{
        let (mem, indices) = &const {
            const L: usize = $crate::macros::__cartesian_product_capacity(&$l, &$r);
            let mut mem = [0; L];
            let mut indices = [(0, 0); $l.len() * $r.len()];
            $crate::macros::__cartesian_product_populate_mem(&$l, &$r, &mut mem, &mut indices);
            (mem, indices)
        };
        let mut strings = [""; $l.len() * $r.len()];
        $crate::macros::__mem_to_str_arr(mem, indices, &mut strings);
        strings
    }};
}

/// Add capitalized versions of each string.
///
/// # Example
///
/// ```
/// use grac::with_capitalized;
///
/// const WC: [&str; 4] = with_capitalized!(["άλφα", "αλλά"]);
/// assert_eq!(WC, ["άλφα", "Άλφα", "αλλά", "Αλλά"]);
/// ```
#[macro_export]
macro_rules! with_capitalized {
    ($arr:expr) => {{
        let (mem, indices) = &const {
            const L: usize = $crate::macros::__str_arr_capacity(&$arr);
            let mut mem = [0; 2 * L];
            let mut indices = [(0, 0); 2 * $arr.len()];
            $crate::macros::__with_capitalized_populate_mem(&$arr, &mut mem, &mut indices);
            (mem, indices)
        };
        let mut strings = [""; 2 * $arr.len()];
        $crate::macros::__mem_to_str_arr(mem, indices, &mut strings);
        strings
    }};
}

// Just syntactic sugar
#[macro_export]
macro_rules! expand_with_capitalized {
    ($l:expr, $r:expr $(,)?) => {
        with_capitalized!(expand!($l, $r))
    };
}

/// Concatenate string arrays.
///
/// # Example
///
/// ```
/// use grac::conc;
///
/// const A: [&str; 2] = ["one", "two"];
/// const B: [&str; 3] = ["3", "4", "5"];
/// const C: [&str; 1] = ["last"];
/// const Z: [&str; 6] = conc!(A, B, C);
/// assert_eq!(Z, ["one", "two", "3", "4", "5", "last"]);
/// ```
#[macro_export]
macro_rules! conc {
    // Base case: If there are only two arrays, concatenate them
    ($l:expr, $r:expr $(,)?) => {{
        let (mem, indices) = &const {
            const L: usize =
                $crate::macros::__str_arr_capacity(&$l) + $crate::macros::__str_arr_capacity(&$r);
            let mut mem = [0; L];
            let mut indices = [(0, 0); $l.len() + $r.len()];
            $crate::macros::__conc_populate_mem(&$l, &$r, &mut mem, &mut indices);
            (mem, indices)
        };
        let mut strings = [""; $l.len() + $r.len()];
        $crate::macros::__mem_to_str_arr(mem, indices, &mut strings);
        strings
    }};

    // Recursive case: If more than two arrays, concatenate the first two, then recurse
    ($l:expr, $r:expr, $($rest:expr),*) => {{
        conc!(conc!($l, $r), $($rest),*)
    }};
}

const fn copy_bytes(x: &[u8], mem: &mut [u8], k: &mut usize) {
    let mut i = 0;
    while i < x.len() {
        mem[*k] = x[i];
        *k += 1;
        i += 1;
    }
}

#[allow(clippy::many_single_char_names)]
const fn copy_bytes_capitalized(x: &[u8], mem: &mut [u8], k: &mut usize) {
    assert!(x.len() >= 2); // Only support Greek
    let mut i = 0;

    let (a, b) = to_uppercase_gr_bytes(x[0], x[1]);
    mem[*k] = a;
    *k += 1;
    i += 1;
    mem[*k] = b;
    *k += 1;
    i += 1;

    while i < x.len() {
        mem[*k] = x[i];
        *k += 1;
        i += 1;
    }
}

pub const fn __with_capitalized_populate_mem(
    x: &[&str],
    mem: &mut [u8],
    indices: &mut [(usize, usize)],
) {
    let mut k = 0;
    let mut i = 0;

    while i < x.len() {
        let k0 = k;
        copy_bytes(x[i].as_bytes(), mem, &mut k);
        indices[2 * i] = (k0, k);
        let k0 = k;
        copy_bytes_capitalized(x[i].as_bytes(), mem, &mut k);
        indices[2 * i + 1] = (k0, k);
        i += 1;
    }
}

pub const fn __conc_populate_mem(
    x: &[&str],
    y: &[&str],
    mem: &mut [u8],
    indices: &mut [(usize, usize)],
) {
    assert!(indices.len() == x.len() + y.len());
    let mut k = 0;
    let mut i = 0;

    while i < x.len() {
        let k0 = k;
        copy_bytes(x[i].as_bytes(), mem, &mut k);
        indices[i] = (k0, k);
        i += 1;
    }

    while i < x.len() + y.len() {
        let k0 = k;
        copy_bytes(y[i - x.len()].as_bytes(), mem, &mut k);
        indices[i] = (k0, k);
        i += 1;
    }
}

#[allow(clippy::many_single_char_names)]
pub const fn __cartesian_product_populate_mem(
    x: &[&str],
    y: &[&str],
    mem: &mut [u8],
    indices: &mut [(usize, usize)],
) {
    assert!(mem.len() == __cartesian_product_capacity(x, y));
    assert!(indices.len() == x.len() * y.len());
    let mut k = 0;
    let mut i = 0;
    while i < x.len() {
        let mut j = 0;
        while j < y.len() {
            let k0 = k;
            copy_bytes(x[i].as_bytes(), mem, &mut k);
            copy_bytes(y[j].as_bytes(), mem, &mut k);
            indices[i * y.len() + j] = (k0, k);
            j += 1;
        }
        i += 1;
    }
}

pub const fn __cartesian_product_capacity(x: &[&str], y: &[&str]) -> usize {
    __str_arr_capacity(x) * y.len() + x.len() * __str_arr_capacity(y)
}

pub const fn __str_arr_capacity(x: &[&str]) -> usize {
    let mut i = 0;
    let mut c = 0;
    while i < x.len() {
        let s = x[i];
        c += s.len();
        i += 1;
    }
    c
}

pub const fn __mem_to_str_arr<'mem>(
    mem: &'mem [u8],
    indices: &[(usize, usize)],
    strings: &mut [&'mem str],
) {
    assert!(indices.len() == strings.len());
    let mut i = 0;
    while i < indices.len() {
        let (l, r) = indices[i];
        let Ok(item) = std::str::from_utf8(mem.split_at(l).1.split_at(r - l).0) else {
            panic!("invalid input to __mem_to_str_arr");
        };
        strings[i] = item;
        i += 1;
    }
}

/// Expects valid modern Greek codepoints.
///
/// Generated with this python script:
///
/// ```python
/// a = "αάβγδεέζηήθιίκλμνξοόπρσςτυύφχψωώ"
/// b = "ΑΆΒΓΔΕΈΖΗΉΘΙΊΚΛΜΝΞΟΌΠΡΣΣΤΥΎΦΧΨΩΏ"
/// for x, y in zip(a, b):
///     xb = x.encode('utf-8')
///     yb = y.encode('utf-8')
///     print(f"({xb[0]}, {xb[1]}) => ({yb[0]}, {yb[1]}),")
/// ```
const fn to_uppercase_gr_bytes(a: u8, b: u8) -> (u8, u8) {
    match (a, b) {
        (206, 177) => (206, 145),
        (206, 172) => (206, 134),
        (206, 178) => (206, 146),
        (206, 179) => (206, 147),
        (206, 180) => (206, 148),
        (206, 181) => (206, 149),
        (206, 173) => (206, 136),
        (206, 182) => (206, 150),
        (206, 183) => (206, 151),
        (206, 174) => (206, 137),
        (206, 184) => (206, 152),
        (206, 185) => (206, 153),
        (206, 175) => (206, 138),
        (206, 186) => (206, 154),
        (206, 187) => (206, 155),
        (206, 188) => (206, 156),
        (206, 189) => (206, 157),
        (206, 190) => (206, 158),
        (206, 191) => (206, 159),
        (207, 140) => (206, 140),
        (207, 128) => (206, 160),
        (207, 129) => (206, 161),
        (207, 131 | 130) => (206, 163), // σς > Σ
        (207, 132) => (206, 164),
        (207, 133) => (206, 165),
        (207, 141) => (206, 142),
        (207, 134) => (206, 166),
        (207, 135) => (206, 167),
        (207, 136) => (206, 168),
        (207, 137) => (206, 169),
        (207, 142) => (206, 143),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase_gr_bytes() {
        let low = "αάβγδεέζηήθιίκλμνξοόπρσςτυύφχψωώ";
        let upp = "ΑΆΒΓΔΕΈΖΗΉΘΙΊΚΛΜΝΞΟΌΠΡΣΣΤΥΎΦΧΨΩΏ";
        for (lower, upper) in low.chars().zip(upp.chars()) {
            let l = lower.to_string().into_bytes();
            let u = upper.to_string().into_bytes();
            let res = to_uppercase_gr_bytes(l[0], l[1]);
            assert_eq!(res, (u[0], u[1]));
        }
    }
}
