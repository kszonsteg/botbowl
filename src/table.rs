use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(PartialEq)]
pub enum Tile {
    #[pyo3(name = "HOME")]
    Home = 1,

    #[pyo3(name = "HOME_TOUCHDOWN")]
    HomeTouchdown = 2,
}
