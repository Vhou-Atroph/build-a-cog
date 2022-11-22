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

#[pyclass]
pub struct Head {
    #[pyo3(get,set)]
    file: String,
    #[pyo3(get,set)]
    node: String, // several heads are stored within each suitx-heads file- it needs to be clarified which node the user would like to use.
    #[pyo3(get,set)]
    texture: Option<String> // head texture may not need to be changed from the node's default
}

// TODO: head list commands for each head model file: phase_4/suitA-heads.bam, phase_4/suitB-heads.bam, phase_3.5/suitC-heads.bam