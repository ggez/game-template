//! specs systems.
use crate::components::*;
use specs::{self, Join};

pub struct MovementSystem;

impl<'a> specs::System<'a> for MovementSystem {
    type SystemData = (
        specs::WriteStorage<'a, Position>,
        specs::WriteStorage<'a, Motion>,
    );

    fn run(&mut self, (mut pos, mut motion): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        for (pos, motion) in (&mut pos, &mut motion).join() {
            pos.0 += motion.velocity;
            motion.velocity += motion.acceleration;
        }
    }
}
