//! This file defines the `World`,
//! as well as some handy utility methods and structs.
//! The `World` contains shared state that will be available
//! to every `Scene`: specs objects, input state, asset cache.

use ggez;
use ggez_goodies::input as ginput;
use ggez::graphics::{Point2, Vector2};
use specs;

use warmy;

use std::path;


use resources::*;
use input;
use log;
use components::*;
use systems::*;

pub struct World {
    pub assets: warmy::Store<ggez::Context>,
    pub input_manager: input::InputState,
    pub specs_world: specs::World,
    pub specs_dispatcher: specs::Dispatcher<'static, 'static>,
}


impl World {
    pub fn new(ctx: &mut ggez::Context, resource_dir: Option<path::PathBuf>) -> Self {
        // We try to bridge the gap between ggez and warmy path
        // handling here; ggez assumes its own absolute paths, warmy
        // assumes system-absolute paths; so, we make warmy look in
        // the specified resource dir (normally
        // $CARGO_MANIFEST_DIR/resources) or the ggez default resource
        // dir.
        let resource_pathbuf: path::PathBuf = match resource_dir {
            Some(s) => s,
            None => ctx.filesystem.get_resources_dir().to_owned(),
        };
        info!(
            log::LOG,
            "Setting up resource path: {:?}",
            resource_pathbuf
        );
        let opt = warmy::StoreOpt::default().set_root(resource_pathbuf);
        let mut store = warmy::Store::new(opt)
            .expect("Could not create asset store?  Does the directory exist?");
        let key = warmy::Key::logical("random_asset_name");
        let _t = store.get::<TestAsset>(&key, ctx);

        let mut w = specs::World::new();
        w.register::<Position>();
        w.register::<Motion>();
        w.register::<Shot>();
        w.register::<Player>();

        w.create_entity()
            .with(Position(Point2::new(0.0, 0.0)))
            .with(Motion {
                velocity: Vector2::new(1.0, 1.0),
                acceleration: Vector2::new(0.0, 0.0),
            })
            .build();

        // ...oooooh, the dispatcher should go in the Scene
        // so every scene can have its own set of systems!
        let dispatcher = specs::DispatcherBuilder::new()
            .add(MovementSystem, "sys_movement", &[])
            .build();

        Self {
            assets: store,
            input_manager: ginput::InputState::new(),
            specs_world: w,
            specs_dispatcher: dispatcher,
        }
    }
}
