use ggez;
use ggez::graphics;
use ggez_goodies::scene;
use log::*;

use crate::input;
use crate::world::World;
use crate::scenes;
use crate::types::*;

pub struct MenuScene {
    title_text: graphics::Text,
    input_text: graphics::Text,
    done: bool,
}

impl MenuScene {
    pub fn new(ctx: &mut ggez::Context, _world: &mut World) -> Self {
        let font = graphics::Font::new(ctx, "/fonts/DejaVuSerif.ttf").unwrap();
        let title_text = graphics::Text::new(("Main Menu", font, 48.0));
        let input_text = graphics::Text::new(("Press Any Key to Start", font, 20.0));

        let done = false;
        MenuScene {
            title_text,
            input_text,
            done,
        }
    }
}

impl scene::Scene<World, input::Event> for MenuScene {
    fn update(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> scenes::Switch {
        if self.done {
            self.done = false;
            scene::SceneSwitch::Push(Box::new(scenes::level::LevelScene::new(ctx, gameworld)))
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, _gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::draw(
            ctx,
            &self.title_text,
            graphics::DrawParam::default().dest(Point2::new(200.0,200.0)),
        )?;
        graphics::draw(
            ctx,
            &self.input_text,
            graphics::DrawParam::default().dest(Point2::new(200.0,300.0)),
        )?;
        Ok(())
    }

    fn name(&self) -> &str {
        "MenuScene"
    }

    fn input(&mut self, _gameworld: &mut World, ev: input::Event, _started: bool) {
        debug!("Input: {:?}", ev);
        self.done = true;
    }
}
