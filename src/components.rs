use ggez::nalgebra as na;
use ggez::graphics::*;
use specs::*;
// use ggez_goodies::asset;
// use ggez_goodies::camera;

use util::*;


/// ///////////////////////////////////////////////////////////////////////
/// Components
/// ///////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Component)]
#[component(VecStorage)]
pub struct Position(pub Point2);


#[derive(Clone, Debug, Component)]
#[component(VecStorage)]
pub struct Motion {
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

// Just a marker that a particular entity is the player.
#[derive(Clone, Debug, Default, Component)]
#[component(NullStorage)]
pub struct Player;

#[derive(Clone, Debug, Default, Component)]
#[component(VecStorage)]
pub struct Shot {
    pub damage: u32,
}


// #[derive(Clone, Debug, Component)]
// #[component(VecStorage)]
// pub struct CImage(pub asset::AssetHandle);
// impl specs::Component for CImage {
//     type Storage = specs::VecStorage<CImage>;
// }

// pub struct CCamera {
//     pub c: camera::Camera,
// }
// impl specs::Component for CCamera {
//     type Storage = specs::HashMapStorage<CCamera>;
// }

// impl CCamera {
//     pub fn new(screen_width: u32, screen_height: u32) -> Self {
//         CCamera { c: camera::Camera::new(screen_width, screen_height, 40.0, 30.0) }
//     }
// }

#[derive(Clone, Debug, Component)]
#[component(HashMapStorage)]
pub struct CBackgroundScroller {
    pub scroll_speed: Vector2,
}

impl CBackgroundScroller {
    pub fn new() -> Self {
        CBackgroundScroller { scroll_speed: Vector2::new(0.0, -0.01) }
    }
}
