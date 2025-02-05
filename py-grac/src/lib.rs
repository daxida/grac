use ::grac as _grac;
use pyo3::prelude::*;

#[pyfunction]
fn syllabify_el(word: &str) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify_el(word))
}

#[pyfunction]
fn syllabify_el_mode(word: &str, synizesis: bool) -> PyResult<Vec<&str>> {
    match synizesis {
        true => Ok(_grac::syllabify_el_mode(word, _grac::Synizesis::Every)),
        false => Ok(_grac::syllabify_el_mode(word, _grac::Synizesis::Never)),
    }
}

#[pyfunction]
fn syllabify_el_mode_at(word: &str, indices: Vec<usize>) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify_el_mode(
        word,
        _grac::Synizesis::Indices(&indices),
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
fn remove_all_diacritics(word: &str) -> PyResult<String> {
    Ok(_grac::remove_all_diacritics(word))
}

#[pyfunction]
fn to_mono(word: &str) -> PyResult<String> {
    Ok(_grac::to_mono(word))
}

#[pyfunction]
fn add_acute_at(word: &str, pos: usize) -> PyResult<String> {
    Ok(_grac::add_acute_at(word, pos))
}

#[pyfunction]
fn remove_diacritic_at(word: &str, pos: usize, diacritic: char) -> PyResult<String> {
    Ok(_grac::remove_diacritic_at(word, pos, diacritic))
}

#[pyfunction]
fn has_diacritic(word: &str, diacritic: char) -> PyResult<bool> {
    Ok(_grac::has_diacritic(word, diacritic))
}

#[pymodule]
fn grac(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(syllabify_el, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_el_mode, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_el_mode_at, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_gr, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_gr_ref, m)?)?;
    m.add_function(wrap_pyfunction!(remove_all_diacritics, m)?)?;
    m.add_function(wrap_pyfunction!(to_mono, m)?)?;
    m.add_function(wrap_pyfunction!(add_acute_at, m)?)?;
    m.add_function(wrap_pyfunction!(remove_diacritic_at, m)?)?;
    m.add_function(wrap_pyfunction!(has_diacritic, m)?)?;
    Ok(())
}
