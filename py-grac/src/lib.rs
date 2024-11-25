use ::grac as _grac;
use pyo3::prelude::*;

#[pyfunction]
fn syllabify(word: &str) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify(word))
}

#[pyfunction]
fn syllabify_ref(word: &str) -> PyResult<Vec<&str>> {
    Ok(_grac::syllabify_ref(word))
}

#[pymodule]
fn grac(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(syllabify, m)?)?;
    m.add_function(wrap_pyfunction!(syllabify_ref, m)?)?;
    Ok(())
}
