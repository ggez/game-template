use ggez;
use ggez::graphics;
use ggez_goodies::scene;
use ggez_goodies::input as ginput;
use specs::Join;
use warmy;

use log;
use input;
use components as c;
use scenes::*;
use world;
use world::World;

pub struct LevelScene {
    done: bool,
    kiwi: warmy::Res<world::Image>,
}


impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        // let kiwi = graphics::Image::new(ctx, "/images/kiwi.png").unwrap();
        let kiwi = world.assets.get::<world::Image>(&warmy::Key::new("images/kiwi.png"), ctx)
            .unwrap();
        LevelScene { 
            done: false,
            kiwi: kiwi,
        }
    }
}

impl scene::Scene<World, input::InputEvent> for LevelScene {
    fn update(&mut self, gameworld: &mut World) -> FSceneSwitch {
        gameworld.specs_dispatcher.dispatch(&mut gameworld.specs_world.res);
        scene::SceneSwitch::None
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let pos = gameworld.specs_world.read::<c::Position>();
        for p in pos.join() {
            graphics::draw(ctx, &(self.kiwi.borrow().0), p.0, 0.0)?;
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::InputEvent, started: bool) {
        debug!(log::LOG, "Input: {:?}", ev);
    }

    fn draw_previous(&self) -> bool {
         false
    }
}

