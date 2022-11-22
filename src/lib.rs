use pyo3::prelude::*;

#[pyclass]
pub struct Suit {
    #[pyo3(get,set)]
    model: String,
    #[pyo3(get,set)]
    torso: String,
    #[pyo3(get,set)]
    arms: String,
    #[pyo3(get,set)]
    legs: String
}