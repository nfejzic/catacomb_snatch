use ggez::*;
use ggez::event::EventHandler;
use ggez::graphics::{window, gfx_objects, Image, DrawParam, Color};
use ggez::graphics::*;
use ggez::conf::{WindowSetup, WindowMode, NumSamples, FullscreenType};
use std::borrow::Borrow;
use std::env;
use std::path;
use std::io::Read;
use std::ptr::null;
use mint::Vector2;
use ggez::graphics::spritebatch::SpriteBatch;
use crate::components::gfx::SpriteSheet;
use crate::components::cutter::SpritesheetCutter;
use crate::components::level::level::Level;

mod components;

struct Game {
    is_running: bool,
    dt: i32,
    level: Level,
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.level.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);
        let scale = 2.0;

        // let img = self.ts.sprites[0][0].img.clone();
        // graphics::draw(ctx, &self.ts.sprites[0][0].img, DrawParam::new().scale(Vector2 { x: scale, y: scale }));
        self.level.draw(ctx);
        graphics::present(ctx);
        Ok(())
    }
}

// game setup
fn main() {
    const WIDTH: f32 = 1280.0;
    const HEIGHT: f32 = 720.0;

// window configuration
    let mut conf = conf::Conf::new();
    conf.window_mode = WindowMode {
        width: WIDTH,
        height: HEIGHT,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        min_height: 0.0,
        max_width: WIDTH,
        max_height: HEIGHT,
        resizable: false,
    };

// window setup -- enable vsync
    conf.window_setup = WindowSetup {
        title: String::from("Catacomb Snatch - Rust"),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_string(),
        srgb: true,
    };


    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Hello_Game", "Nadir Fejzic")
        .conf(conf)
        .build()
        .unwrap();

    filesystem::mount(ctx, "./resources".as_ref(), true);

    // let level = Level::new(ctx, 32, 32);
    let mut level = Level::from_image(ctx, "/level/level1.bmp");
    let mut state = &mut Game { is_running: true, dt: 0, level };

    event::run(ctx, event_loop, state).unwrap();
}
