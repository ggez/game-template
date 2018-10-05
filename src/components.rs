//use ggez::nalgebra as na;
use ggez::graphics::*;
use specs::*;

//use util::*;

/// ///////////////////////////////////////////////////////////////////////
/// Components
/// ///////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Point2);

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Motion {
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

// Just a marker that a particular entity is the player.
#[derive(Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Clone, Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Shot {
    pub damage: u32,
}

#[derive(Clone, Debug, Component)]
#[storage(HashMapStorage)]
pub struct CBackgroundScroller {
    pub scroll_speed: Vector2,
}

impl CBackgroundScroller {
    //pub fn new() -> Self {
    //    CBackgroundScroller { scroll_speed: Vector2::new(0.0, -0.01) }
    //}
}
