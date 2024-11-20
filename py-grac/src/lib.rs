use ::grac as _grac;
use greek_syllables as gs;
use pyo3::prelude::*;

#[pyfunction]
fn syllabify(word: &str) -> PyResult<Vec<String>> {
    Ok(_grac::syllabify(word))
}

#[pyfunction]
fn syllabify_2(word: &str) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify_2(word))
}

#[pyfunction]
fn syllabify_3(word: &str) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify_3(word))
}

#[pyfunction]
fn syllables(word: &str) -> PyResult<Vec<&str>> {
    Ok(gs::syllables(word))
}

#[pymodule]
fn grac(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(syllabify, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_2, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_3, m)?)?;
    m.add_function(wrap_pyfunction!(syllables, m)?)?;
    Ok(())
}
