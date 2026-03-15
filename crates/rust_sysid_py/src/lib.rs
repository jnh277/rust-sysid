use pyo3::prelude::*;
use ::rust_sysid as sysid;

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    sysid::add(a, b)
}

#[pymodule]
fn rust_sysid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
