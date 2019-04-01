use crate::types::*;

use specs::*;
use specs_derive::*;

// ///////////////////////////////////////////////////////////////////////
// Components
// ///////////////////////////////////////////////////////////////////////

/// A position in the game world.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Point2);

/// Motion in the game world.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Motion {
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

/// Just a marker that a particular entity is the player.
#[derive(Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Clone, Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Shot {
    pub damage: u32,
}

pub fn register_components(specs_world: &mut World) {
    specs_world.register::<Position>();
    specs_world.register::<Motion>();
    specs_world.register::<Shot>();
    specs_world.register::<Player>();
}
