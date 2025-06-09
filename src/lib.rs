use pyo3::prelude::{pymodule, Bound, PyModule, PyModuleMethods, PyResult};

#[pymodule]
fn rust_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
