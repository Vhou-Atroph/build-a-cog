use pyo3::prelude::*;

#[pyclass]
pub struct Cog {
    #[pyo3(get,set)]
    suit: Suit,
    #[pyo3(get,set)]
    head: Head,
    #[pyo3(get,set)]
    animation: Animation,
    #[pyo3(get,set)]
    hands: (f32,f32,f32,f32), // A Color tuple for Panda3D. It is in the format of (Red,Green,Blue,Alpha)
}

#[pyclass]
#[derive(Clone)]
pub struct Suit { // did you know: the original name for cogs was going to be suits
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
#[derive(Clone)]
pub struct Head {
    #[pyo3(get,set)]
    file: String,
    #[pyo3(get,set)]
    node: String, // several heads are stored within each suitx-heads file- it needs to be clarified which node the user would like to use.
    #[pyo3(get,set)]
    texture: Option<String> // head texture may not need to be changed from the node's default
}

// TODO: head list commands for each head model file: phase_4/suitA-heads.bam, phase_4/suitB-heads.bam, phase_3.5/suitC-heads.bam

/// Taken directly from adopt_a_doodle.
#[pyclass]
#[derive(Clone)]
pub struct Animation {
    #[pyo3(get,set)]
    file: String,
    #[pyo3(get,set)]
    anim_loop: bool,
    #[pyo3(get,set)]
    loop_from: Option<u64>,
    #[pyo3(get,set)]
    loop_to: Option<u64>,
    #[pyo3(get,set)]
    loop_restart: Option<u64>,
    #[pyo3(get,set)]
    pose: bool, // Takes priority over anim_loop - if this is set to true then the actor will be stuck in a pose.
    #[pyo3(get,set)]
    pose_frame: Option<u64> // Necessary parameter if pose is true. Sets the frame the cog will be posed at.
}

#[pymethods]
impl Animation {

    /// Creates a new Animation struct.
    #[new]
    pub fn new(file:String,anim_loop:bool,loop_from:Option<u64>,loop_to:Option<u64>,loop_restart:Option<u64>,pose:bool,pose_frame:Option<u64>) -> Self {
        Self {file,anim_loop,loop_from,loop_to,loop_restart,pose,pose_frame}
    }
}