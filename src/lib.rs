use pyo3::prelude::{pymodule, Bound, PyAnyMethods, PyModule, PyModuleMethods, PyResult};

mod table;

const LIB_NAME: &str = "rust_core";

#[pymodule]
fn rust_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_table_module(m)?;

    Ok(())
}

fn register_table_module(core_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let table_mod = PyModule::new(core_module.py(), "table")?;
    table_mod.add_class::<table::Tile>()?;

    core_module.add_submodule(&table_mod)?;
    core_module
        .py()
        .import("sys")?
        .getattr("modules")?
        .set_item(format!("botbowl.{}.table", LIB_NAME), table_mod)
}
