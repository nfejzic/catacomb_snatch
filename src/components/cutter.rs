use crate::components::gfx::{SpriteSheet, Sprite};
use ggez::Context;

pub trait SpritesheetCutter {
    fn load_spritesheet_default(ctx: &mut Context, path: &str) -> SpriteSheet;
    fn cut_default(&self, ctx: &mut Context) -> Vec<Vec<Sprite>>;
    fn cut_sized(&self, ctx: &mut Context, w: i32, h: i32) -> Vec<Vec<Sprite>>;
    fn cut(&self, ctx: &mut Context, x_begin: i32, x_end: i32, y_begin: i32, y_end: i32, w: i32, h: i32) -> Vec<Vec<Sprite>>;
}
