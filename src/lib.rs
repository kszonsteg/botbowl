use pyo3::prelude::{pymodule, Bound, PyAnyMethods, PyModule, PyModuleMethods, PyResult};
use pyo3::wrap_pymodule;

mod table;

#[pymodule]
fn rust_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let table_submodule = PyModule::new(m.py(), "table")?;

    table_submodule.add_wrapped(wrap_pymodule!(table::table))?;

    m.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("botbowl.rust_core", table_submodule)?;

    Ok(())
}
