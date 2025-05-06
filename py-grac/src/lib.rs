use ::grac as _grac;
use pyo3::prelude::*;

#[pyfunction]
fn syllabify(word: &str) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify(word).to_vec())
}

#[pyfunction]
fn syllabify_with_merge(word: &str, merge: bool) -> PyResult<Vec<&str>> {
    let merge = if merge {
        _grac::Merge::Every
    } else {
        _grac::Merge::Never
    };
    let syllables = _grac::syllabify_with_merge(word, merge);
    Ok(syllables.to_vec())
}

#[pyfunction]
fn syllabify_with_merge_at(word: &str, indices: Vec<usize>) -> PyResult<Vec<&str>> {
    let merge = _grac::Merge::Indices(indices);
    let syllables = _grac::syllabify_with_merge(word, merge);
    Ok(syllables.to_vec())
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
    m.add_function(wrap_pyfunction!(syllabify, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_with_merge, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_with_merge_at, m)?)?;
    m.add_function(wrap_pyfunction!(has_diacritic, m)?)?;
    m.add_function(wrap_pyfunction!(remove_all_diacritics, m)?)?;
    m.add_function(wrap_pyfunction!(remove_diacritic_at, m)?)?;
    m.add_function(wrap_pyfunction!(add_acute_at, m)?)?;
    m.add_function(wrap_pyfunction!(to_monotonic, m)?)?;
    Ok(())
}
