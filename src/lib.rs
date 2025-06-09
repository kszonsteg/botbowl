use pyo3::prelude::{pymodule, Bound, PyAnyMethods, PyModule, PyModuleMethods, PyResult};

mod table;

#[pymodule]
fn rust_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_child_module(m)?;

    Ok(())
}

fn register_child_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "table")?;
    child_module.add_class::<table::Tile>()?;

    parent_module.add_submodule(&child_module)?;
    parent_module
        .py()
        .import("sys")?
        .getattr("modules")?
        .set_item("botbowl.rust_core.table", child_module)
}
