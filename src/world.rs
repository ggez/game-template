use crate::{components, input, resources, util};

use log::*;
use specs::{self, world::Builder};
use warmy;

use std::path;

pub struct World {
    pub resources: resources::Store,
    pub input: input::State,
    pub specs_world: specs::World,
}

impl World {
    pub fn new(resource_dir: &path::Path) -> Self {
        // We to bridge the gap between ggez and warmy path
        // handling here; ggez assumes its own absolute paths, warmy
        // assumes system-absolute paths; so, we make warmy look in
        // the specified resource dir (normally
        // $CARGO_MANIFEST_DIR/resources) or the ggez default resource
        // dir.
        //
        // TODO: ...though this doesn't SEEM to quite do reloading right, so
        // work on it more.
        info!("Setting up resource path: {:?}", resource_dir);
        let opt = warmy::StoreOpt::default().set_root(resource_dir);
        let store = warmy::Store::new(opt)
            .expect("Could not create asset store?  Does the directory exist?");

        let mut w = specs::World::new();
        components::register_components(&mut w);

        let mut the_world = Self {
            resources: store,
            input: input::State::new(),
            specs_world: w,
        };

        // Make a test entity.
        the_world
            .specs_world
            .create_entity()
            .with(components::Position(util::point2(0.0, 0.0)))
            .with(components::Motion {
                velocity: util::vec2(1.0, 1.0),
                acceleration: util::vec2(0.0, 0.0),
            })
            .build();

        the_world
    }
}
