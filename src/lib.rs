use pyo3::prelude::*;

/// A boring robo-businessman that just can't take a joke! Created by Gyro Gearloose for the benefit of Toontown, they now seek to conquer it.
/// Here's an explanation of each field:
/// ```rust
/// use rustycog::*;
/// 
/// fn makeacog() -> Cog {
///     Cog {
///         suit: Suit { // See the docs for the Suit struct.
///             model: String::from("tt_a_ene_cga_zero.bam"),
///             torso: String::from("phase_3.5/maps/c_blazer.jpg"),
///             arms: String::from("phase_3.5/maps/c_sleeve.jpg"),
///             legs: String::from("phase_3.5/maps/c_leg.jpg")
///         },
///         head: Head { // See the docs for the Head struct.
///             file: String::from("phase_4/models/char/suitA-heads.bam"),
///             node: String::from("yesman"),
///             texture: None,
///             color: None
///         },
///         animation: Animation { // See the docs for the Animation struct.
///             file: String::from("phase_5/models/char/tt_a_ene_cga_song-and-dance.bam"),
///             anim_loop: true,
///             loop_from: None,
///             loop_to: None,
///             loop_restart: None,
///             pose: false,
///             pose_frame: None
///         },
///         hands: (0.95,0.75,0.75,1.0) // The color field requires a struct containing four 32-bit floating point values of values 0 through 1. The first three values are for Red, Green, and Blue content. The fourth value is the Alpha value. In this example, the color is the Bossbot suit's hand color as referenced from Anesidora.
///     }
/// }
/// ```
/// However, since this is a Python module, you probably won't be implementing this struct like this in Rust! Here's how the same cog would look in Python:
/// ```python
/// from rustycog import *
/// 
/// let yesman = Cog(suit=Suit(model="tt_a_ene_cga_zero.bam",
///         torso="phase_3.5/maps/c_blazer.jpg",
///         arms="phase_3.5/maps/c_sleeve.jpg",
///         legs="phase_3.5/maps/c_leg.jpg"
///     ),
///     head=Head(file="phase_4/models/char/suitA-heads.bam",
///         node="yesman",
///         texture=None,
///         color=None
///     ),
///     animation=Animation(file="phase_5/models/char/tt_a_ene_cga_song-and-dance.bam",
///         anim_loop=True,
///         loop_from=None,
///         loop_to=None,
///         loop_restart=None,
///         pose=False,
///         pose_frame=None
///     ),
///     hands=(0.95,0.75,0.75,1.0)
/// )
/// ```
/// If using rustycog through build_a_cog, the Cog struct will be accessible through `build_a_cog.Cog`.
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

#[pymethods]
impl Cog {

    /// Creates a new Cog struct.
    #[new]
    pub fn new(suit:Suit,head:Head,animation:Animation,hands:(f32,f32,f32,f32)) -> Self {
        Self {suit,head,animation,hands}
    }
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

#[pymethods]
impl Suit {

    /// Creates a new Suit struct.
    #[new]
    pub fn new(model:String,torso:String,arms:String,legs:String) -> Self {
        Self {model,torso,arms,legs}
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Head {
    #[pyo3(get,set)]
    file: String,
    #[pyo3(get,set)]
    node: String, // several heads are stored within each suitx-heads file- it needs to be clarified which node the user would like to use.
    #[pyo3(get,set)]
    texture: Option<String>, // head texture may not need to be changed from the node's default
    #[pyo3(get,set)]
    color: Option<String> // on the chance you need to recolor the head (cold caller)
}

// TODO: head list commands for each head model file: phase_4/suitA-heads.bam, phase_4/suitB-heads.bam, phase_3.5/suitC-heads.bam

/// Taken directly from adopt_a_doodle.
/// /// An animation that the cog will perform.
/// Here's an explanation for each field:
/// ```rust
/// use rustycog::*;
/// 
/// fn exampleanimation() -> Animation {
///     Animation {
///     file: String::from("phase_5/models/char/tt_a_ene_cga_song-and-dance.bam"), // A String value representing a file path that leads to a cog's animation.
///     anim_loop: true, // A boolean that determines whether or not the cog's animation will loop. Here, it is set to true.
///     loop_from: None, // An Option<u64> value that determines from which initial frame the animation will loop.
///     loop_to: None, // An Option<u64> value that determines to which final frame the animation will loop.
///     loop_restart: None, // An Option<u64> value that determines from which frame the cog's animation will restart.
///     pose: false, // A boolean that determines whether or not the cog's animation will be frozen into a pose. This takes priority over anim_loop!
///     pose_frame: None // An Option<u64> value that determines which frame the cog will be posed at.
///     }
/// }
/// ```
/// However, since this is a Python module, you probably won't be implementing this struct like this in Rust! Here's how the same animation would look in Python:
/// ```python
/// from rustycog import *
/// 
/// exampleanimation = Animation(file="phase_5/models/char/tt_a_ene_cga_song-and-dance.bam",
///     anim_loop=True,
///     loop_from=None,
///     loop_to=None,
///     loop_restart=None,
///     pose=False,
///     pose_frame=None)
/// ```
/// If using rustycog through build_a_cog, the Pattern struct will be accessible through `build_a_cog.Animation`.
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