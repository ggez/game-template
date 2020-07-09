use ggez_goodies::scene;

use crate::input;
use crate::world::World;

pub mod level;
pub mod menu;

// Shortcuts for our scene type.
pub type Switch = scene::SceneSwitch<World, input::Event>;
pub type Stack = scene::SceneStack<World, input::Event>;
// Useless, since you can't impl type aliases.  :|
//pub trait Scene = scene::Scene<World, input::Event>;
