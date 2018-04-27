//! Game setup and very basic main loop.
//! All the actual work gets done in the Scene.

#[macro_use]
extern crate failure;
extern crate ggez;
extern crate ggez_goodies;
#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;
extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate warmy;

use ggez::*;
use ggez::conf;
use ggez::event;

use ggez::event::*;
use ggez::graphics;
use ggez::timer;
// use ggez_goodies::camera;

use std::path;

// Modules that define actual content
mod components;
mod scenes;
mod systems;
mod world;

// Modules that define utility stuff.
mod error;
mod input;
mod resources;
//mod log;
mod util;


/// Function to set up logging.
/// We write all debug messages (which will be a log)
/// to both stdout and a log file.
/// See the ggez logging example for a more sophisticated
/// setup, we should incorporate some of that here.
///
/// TODO: Don't output colors to the log file.
fn setup_logger() -> Result<(), fern::InitError> {
    use fern::colors::{Color, ColoredLevelConfig};
    // I'm used to Python's logging colors and format,
    // so let's do something like that.
    let colors = ColoredLevelConfig::default()
        .info(Color::Green)
        .debug(Color::BrightMagenta)
        .trace(Color::BrightBlue);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{:<14}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()).to_string(),
                record.target(),
                message
            ))
        })
        // gfx_device_gl is very chatty on info loglevel, so
        // filter that a bit more strictly.
        .level_for("gfx_device_gl", log::LevelFilter::Warn)
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(std::fs::OpenOptions::new()
               .write(true)
               .create(true)
               .truncate(true)
               .open("debug.log")?)
        .apply()?;
    Ok(())
}

/// Main game state.  This holds all our STUFF,
/// but most of the actual game data are
/// in `Scenes`, and the `FSceneStack` contains them
/// plus global game state.
pub struct MainState {
    scenes: scenes::FSceneStack,
    input_binding: input::InputBinding,
}

impl MainState {
    pub fn new(resource_dir: Option<path::PathBuf>, ctx: &mut Context) -> Self {
        let world = world::World::new(ctx, resource_dir.clone());
        let mut scenestack = scenes::FSceneStack::new(ctx, world);
        let initial_scene = Box::new(scenes::level::LevelScene::new(ctx, &mut scenestack.world));
        scenestack.push(initial_scene);
        MainState {
            scenes: scenestack,
            input_binding: input::create_input_binding(),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.scenes.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.scenes.draw(ctx);
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.input(ev, true);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.input(ev, false);
        }
    }
}

pub fn main() {
    setup_logger()
        .expect("Could not set up logging!");
    let mut cb = ContextBuilder::new("game-template", "ggez")
        .window_setup(conf::WindowSetup::default().title("game-template"))
        .window_mode(conf::WindowMode::default().dimensions(800, 600));

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we we look in the cargo project for files.
    // And save it so we can feed there result into warmy
    let cargo_path: Option<path::PathBuf> = option_env!("CARGO_MANIFEST_DIR").map(|env_path| {
        let mut res_path = path::PathBuf::from(env_path);
        res_path.push("resources");
        res_path
    });
    // If we have such a path then add it to the context builder too
    // (modifying the cb from inside a closure gets sticky)
    if let Some(ref s) = cargo_path {
        cb = cb.add_resource_path(s);
    }

    let ctx = &mut cb.build().unwrap();

    let state = &mut MainState::new(cargo_path, ctx);
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
