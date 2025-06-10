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
    table_mod.add_class::<table::BBDieResult>()?;
    table_mod.add_class::<table::RollType>()?;
    table_mod.add_class::<table::OutcomeType>()?;
    table_mod.add_class::<table::PlayerActionType>()?;
    table_mod.add_class::<table::PhysicalState>()?;
    table_mod.add_class::<table::CasualtyEffect>()?;
    table_mod.add_class::<table::CasualtyType>()?;
    table_mod.add_class::<table::ActionType>()?;
    table_mod.add_class::<table::WeatherType>()?;
    table_mod.add_class::<table::SkillCategory>()?;
    table_mod.add_class::<table::Skill>()?;
    table_mod.add_class::<table::PassDistance>()?;
    table_mod.add_class::<table::Rules>()?;

    core_module.add_submodule(&table_mod)?;
    core_module
        .py()
        .import("sys")?
        .getattr("modules")?
        .set_item(format!("botbowl.{}.table", LIB_NAME), table_mod)
}
