//! This file defines the `World`,
//! as well as some handy utility methods and structs.
//! The `World` contains shared state that will be available
//! to every `Scene`: specs objects, input state, asset cache.

use ggez;
use ggez_goodies::input as ginput;
use ggez::graphics;
use ggez::graphics::{Point2, Vector2};
use specs;
use specs::Join;

use warmy;
use failure;
use failure::Fail;

use std::path;

use input;
use log;
use components::*;

#[derive(Debug, Fail)]
#[fail(display = "ggez error: {:?}", err)]
pub struct GgezError {
    err: ggez::GameError,
}

impl From<ggez::GameError> for GgezError {
    fn from(err: ggez::GameError) -> Self {
        Self { err }
    }
}

/// warmy deals with absolute paths.
/// ggez deals with sorta faux-absolute paths.
/// This takes an absolute path, and a warmy store
/// that aims at a ggez resources directory,
/// and returns a path that can be used to load
/// the targeted thing inside ggez.
fn warmy_to_ggez_path<C>(path: &path::Path, store: &warmy::Store<C>) -> path::PathBuf {
    assert!(path.is_absolute());
    let stripped_path = path.strip_prefix(store.root())
        .expect("path is outside of the warmy store?");
    path::Path::new("/").join(stripped_path)
}

struct TestAsset;

impl<C> warmy::Load<C> for TestAsset {
    type Error = failure::Compat<GgezError>;
    fn from_fs<P>(
        path: P,
        store: &mut warmy::Store<C>,
        _ctx: &mut C,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    where
        P: AsRef<path::Path>,
    {
        debug!(log::LOG, "Attempting to load: {:?}", path.as_ref());
        let ggez_path = warmy_to_ggez_path(path.as_ref(), store);
        debug!(log::LOG, "ggez path is now: {:?}", ggez_path);
        Ok(TestAsset.into())
    }
}

pub struct Image(pub ggez::graphics::Image);
impl warmy::Load<ggez::Context> for Image {
    type Error = failure::Compat<GgezError>;
    fn from_fs<P>(
        path: P,
        store: &mut warmy::Store<ggez::Context>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    where
        P: AsRef<path::Path>,
    {
        debug!(log::LOG, "Attempting to load: {:?}", path.as_ref());
        let ggez_path = warmy_to_ggez_path(path.as_ref(), store);
        graphics::Image::new(ctx, &ggez_path)
            .map(|x| warmy::Loaded::from(Image(x)))
            .map_err(|e| GgezError::from(e).compat())
    }
}

pub struct World {
    pub assets: warmy::Store<ggez::Context>,
    pub input_manager: input::InputState,
    pub specs_world: specs::World,
    pub specs_dispatcher: specs::Dispatcher<'static, 'static>,
}

struct MovementSystem;

impl<'a> specs::System<'a> for MovementSystem {
    type SystemData = (
        specs::WriteStorage<'a, Position>,
        specs::ReadStorage<'a, Motion>,
    );

    fn run(&mut self, (mut pos, motion): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        for (pos, motion) in (&mut pos, &motion).join() {
            pos.0 += motion.velocity;
        }
    }
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
            "Setting up resource path path: {:?}",
            resource_pathbuf
        );
        let opt = warmy::StoreOpt::default().set_root(resource_pathbuf);
        let mut store = warmy::Store::new(opt)
            .expect("Could not create asset store?  Does the directory exist?");
        let _t = store.get::<TestAsset>(&warmy::Key::new("foothing"), ctx);

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
