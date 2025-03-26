use ::grac as _grac;
use pyo3::prelude::*;

#[pyfunction]
fn syllabify_el(word: &str) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify_el(word))
}

#[pyfunction]
fn syllabify_el_mode(word: &str, merge: bool) -> PyResult<Vec<&str>> {
    match merge {
        true => Ok(_grac::syllabify_el_mode(word, _grac::Merge::Every)),
        false => Ok(_grac::syllabify_el_mode(word, _grac::Merge::Never)),
    }
}

#[pyfunction]
fn syllabify_el_mode_at(word: &str, indices: Vec<usize>) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify_el_mode(
        word,
        _grac::Merge::Indices(&indices),
    ))
}

#[pyfunction]
fn syllabify_gr(word: &str) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify_gr(word))
}

#[pyfunction]
fn syllabify_gr_ref(word: &str) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify_gr_ref(word))
}

#[pyfunction]
fn has_diacritic(word: &str, diacritic: char) -> PyResult<bool> {
    Ok(_grac::has_diacritic(word, diacritic))
}

#[pyfunction]
fn remove_all_diacritics(word: &str) -> PyResult<String> {
    Ok(_grac::remove_all_diacritics(word))
}

#[pyfunction]
fn remove_diacritic_at(word: &str, pos: usize, diacritic: char) -> PyResult<String> {
    Ok(_grac::remove_diacritic_at(word, pos, diacritic))
}

#[pyfunction]
fn add_acute_at(word: &str, pos: usize) -> PyResult<String> {
    Ok(_grac::add_acute_at(word, pos))
}

#[pyfunction]
fn to_monotonic(word: &str) -> PyResult<String> {
    Ok(_grac::to_monotonic(word))
}

#[pymodule]
fn grac(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(syllabify_el, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_el_mode, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_el_mode_at, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_gr, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_gr_ref, m)?)?;
    m.add_function(wrap_pyfunction!(has_diacritic, m)?)?;
    m.add_function(wrap_pyfunction!(remove_all_diacritics, m)?)?;
    m.add_function(wrap_pyfunction!(remove_diacritic_at, m)?)?;
    m.add_function(wrap_pyfunction!(add_acute_at, m)?)?;
    m.add_function(wrap_pyfunction!(to_monotonic, m)?)?;
    Ok(())
}
