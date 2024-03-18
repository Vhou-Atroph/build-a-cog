use pyo3::prelude::*;

/// Module for basic classes in build-a-cog. This module is implemented in Rust.
#[pymodule]
fn build_a_cog(_: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Cog>()?;
    m.add_class::<Suit>()?;
    m.add_class::<Head>()?;
    m.add_class::<Animation>()?;
    m.add_function(wrap_pyfunction!(suita_heads, m)?)?;
    m.add_function(wrap_pyfunction!(suitb_heads, m)?)?;
    m.add_function(wrap_pyfunction!(suitc_heads, m)?)?;
    Ok(())
}

/// A boring robo-businessman that just can't take a joke! Created by Gyro Gearloose for the benefit of Toontown, they now seek to conquer it.
/// Here's an explanation of each field:
/// ```rust
/// use build_a_cog::*;
/// 
/// fn makeacog() -> Cog {
///     Cog {
///         suit: Suit { // See the docs for the Suit struct.
///             model: String::from("tt_a_ene_cga_zero.bam"),
///             sigil: Some(String::from("CorpIcon"))
///             torso: String::from("phase_3.5/maps/c_blazer.jpg"),
///             arms: String::from("phase_3.5/maps/c_sleeve.jpg"),
///             legs: String::from("phase_3.5/maps/c_leg.jpg"),
///             hands: (0.95,0.75,0.75,1.0)
///         },
///         head: Some(Head { // See the docs for the Head struct.
///             file: String::from("phase_4/models/char/suitA-heads.bam"),
///             node: String::from("yesman"),
///             texture: None,
///             color: None
///         }),
///         animation: Animation { // See the docs for the Animation struct.
///             file: String::from("phase_5/models/char/tt_a_ene_cga_song-and-dance.bam"),
///             anim_loop: true,
///             loop_from: None,
///             loop_to: None,
///             loop_restart: None,
///             pose: false,
///             pose_frame: None
///         }
///     }
/// }
/// ```
/// However, since this is a Python module, you probably won't be implementing this struct like this in Rust! Here's how the same cog would look in Python:
/// ```python
/// from build_a_cog import *
/// 
/// let yesman = Cog(suit=Suit(model="tt_a_ene_cga_zero.bam",
///         sigil="CorpIcon"
///         torso="phase_3.5/maps/c_blazer.jpg",
///         arms="phase_3.5/maps/c_sleeve.jpg",
///         legs="phase_3.5/maps/c_leg.jpg",
///         hands=(0.95,0.75,0.75,1.0)
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
///     )
/// )
/// ```
/// If using build_a_cog through build_a_cog, the Cog struct will be accessible through `build_a_cog.Cog`.
#[pyclass]
pub struct Cog {
    #[pyo3(get,set)]
    suit: Suit,
    #[pyo3(get,set)]
    head: Option<Head>, // option is provided in case a user wishes to use a custom cog head.
    #[pyo3(get,set)]
    animation: Animation,
}

#[pymethods]
impl Cog {

    /// Creates a new Cog struct.
    #[new]
    pub fn new(suit:Suit,animation:Animation,head:Option<Head>) -> Self {
        Self {suit,head,animation}
    }
}

/// The model and texture for the suit that a Cog wears.
/// Here's an explanation of each field:
/// ```rust
/// use build_a_cog::*;
/// 
/// fn makeasuit() -> Suit {
///     Suit {
///         model: String::from("tt_a_ene_cga_zero.bam"), // This references the suit model the cog wears. In this example, the cog wears a type A suit.
///         sigil: Some(String::from("CorpIcon")), // This references the Bossbot sigil present on Bossbot Cog suits.
///         torso=String::from("phase_3.5/maps/c_blazer.jpg"), // This is the texture file for the suit's torso. This uses the Bossbot suit texture.
///         arms: String::from("phase_3.5/maps/c_sleeve.jpg"), // This is the texture file for the suit's arms. This uses the Bossbot suit texture.
///         legs: String::from("phase_3.5/maps/c_leg.jpg"), // This is the texture file for the suit's legs. This uses the Bossbot suit texture.
///         hands: (0.95,0.75,0.75,1.0) // The color field requires a struct containing four 32-bit floating point values of values 0 through 1. The first three values are for Red, Green, and Blue content. The fourth value is the Alpha value. In this example, the color is the Bossbot suit's hand color as referenced from Anesidora.
///     }
/// }
/// ```
/// However, since this is a Python module, you probably won't be implementing this struct like this in Rust! Here's how the same cog would look in Python:
/// ```python
/// from build_a_cog import *
/// 
/// boss_suit = Suit(model="tt_a_ene_cga_zero.bam",
///     sigil="CorpIcon"
///     torso="phase_3.5/maps/c_blazer.jpg",
///     arms="phase_3.5/maps/c_sleeve.jpg",
///     legs="phase_3.5/maps/c_leg.jpg",
///     hands=(0.95,0.75,0.75,1.0)
/// )
/// ```
/// If using build_a_cog through build_a_cog, the Suit struct will be accessible through `build_a_cog.Suit`.
#[pyclass]
#[derive(Clone)]
pub struct Suit { // did you know: the original name for cogs was going to be suits
    #[pyo3(get,set)]
    model: String,
    #[pyo3(get,set)]
    sigil: Option<String>, // this can have four values corresponding to each department: SalesIcon CorpIcon LegalIcon, and MoneyIcon.
    #[pyo3(get,set)]
    torso: String,
    #[pyo3(get,set)]
    arms: String,
    #[pyo3(get,set)]
    legs: String,
    #[pyo3(get,set)]
    hands: (f32,f32,f32,f32), // A Color tuple for Panda3D. It is in the format of (Red,Green,Blue,Alpha)
}

#[pymethods]
impl Suit {

    /// Creates a new Suit struct.
    #[new]
    pub fn new(model:String,torso:String,arms:String,legs:String,hands:(f32,f32,f32,f32),sigil:Option<String>) -> Self {
        Self {model,sigil,torso,arms,legs,hands}
    }
}

/// The head of a cog.
/// Here's an explanation for each field:
/// ```rust
/// use build_a_cog::*;
/// 
/// fn makeahead() -> Head {
///     Head {
///         file: String::from("phase_4/models/char/suitA-heads.bam"), // This is a file that links to a group of heads for a particular suit type. This file is for suit A heads.
///         node: String::from("yesman"), // This is the head node we want to show for the cog. This node is Yesman/Mr. Hollywood head. Note that the Glad Hander's head, despite using the same texture, is in suitC-heads.bam.
///         texture: None, // This is an optional parameter that can define a texture we want the suit head to have. In this case, it is None since we want the Yesman's head texture.
///         color=None // This is an optional parameter that can change the color of a cog's head. This is most notable with the Cold Caller, which has a darker blue head than the Short Change. In this case, we do not need to change the texture.
///     }
/// }
/// ```
/// However, since this is a Python module, you probably won't be implementing this struct like this in Rust! Here's how the same head would look in Python:
/// ```python
/// from build_a_cog import *
/// 
/// head=Head(file="phase_4/models/char/suitA-heads.bam",
///     node="yesman",
///     texture=None,
///     color=None
/// )
/// ```
/// If using build_a_cog through build_a_cog, the Head struct will be accessible through `build_a_cog.Head`.
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

#[pymethods]
impl Head {

    /// Creates a new Head struct.
    #[new]
    pub fn new(file:String,node:String,texture:Option<String>,color:Option<String>) -> Self {
        Self {file,node,texture,color}
    }
}

/// Returns a vector containing the names of the nodes in suitA-heads.bam
#[pyfunction]
pub fn suita_heads() -> Vec<String> {
    vec![String::from("twoface"), // also used for double talker + the mingler
    String::from("yesman"), // also used for robber baron + mr. hollywood
    String::from("pennypincher"),
    String::from("numbercruncher"), // also used for name dropper
    String::from("legaleagle"),
    String::from("headhunter"),
    String::from("bigwig"),
    String::from("bigcheese"),
    String::from("backstabber")]
}

/// Returns a vector containing the names of the nodes in suitB-heads.bam
#[pyfunction]
pub fn suitb_heads() -> Vec<String> {
    vec![String::from("telemarketer"), // also used for spin doctor
    String::from("pencilpusher"),
    String::from("movershaker"), // also used for bloodsucker
    String::from("loanshark"),
    String::from("beancounter"), // also used for downsizer
    String::from("ambulancechaser")]
}

/// Returns a vector containing the names of the nodes in suitC-heads.bam
#[pyfunction]
pub fn suitc_heads() -> Vec<String> {
    vec![String::from("tightwad"), // also used by bottom feeder
    String::from("moneybags"),
    String::from("micromanager"),
    String::from("gladhander"),
    String::from("flunky"), // also used by corporate raider- this head does not come with the funky glasses.
    String::from("coldcaller") // honestly this is kinda fucked up. despite being named cold caller, by default this is the short change head. if you want the cold caller head you need to set the color to (0.25, 0.35, 1.0, 1.0).
    ]
}

/// Taken directly from adopt_a_doodle.
/// /// An animation that the cog will perform.
/// Here's an explanation for each field:
/// ```rust
/// use build_a_cog::*;
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
/// from build_a_cog import *
/// 
/// exampleanimation = Animation(file="phase_5/models/char/tt_a_ene_cga_song-and-dance.bam",
///     anim_loop=True,
///     loop_from=None,
///     loop_to=None,
///     loop_restart=None,
///     pose=False,
///     pose_frame=None)
/// ```
/// If using build_a_cog through build_a_cog, the Animation struct will be accessible through `build_a_cog.Animation`.
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
    pub fn new(file:String,anim_loop:bool,pose:bool,pose_frame:Option<u64>,loop_from:Option<u64>,loop_to:Option<u64>,loop_restart:Option<u64>) -> Self {
        Self {file,anim_loop,loop_from,loop_to,loop_restart,pose,pose_frame}
    }
}