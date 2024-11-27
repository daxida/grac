use ::grac as _grac;
use pyo3::prelude::*;

#[pyfunction]
fn syllabify_el(word: &str) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify_el(word))
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
fn remove_accents(word: &str) -> PyResult<String> {
    Ok(_grac::remove_accents(word))
}

#[pyfunction]
fn to_mono(word: &str) -> PyResult<String> {
    Ok(_grac::to_mono(word))
}

#[pymodule]
fn grac(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(syllabify_el, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_gr, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_gr_ref, m)?)?;
    m.add_function(wrap_pyfunction!(remove_accents, m)?)?;
    m.add_function(wrap_pyfunction!(to_mono, m)?)?;
    Ok(())
}
