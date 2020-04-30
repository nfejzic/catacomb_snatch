use ggez::{filesystem, graphics, Context, GameResult};
use ggez::graphics::{Image, ImageGeneric, GlBackendSpec, WHITE, FilterMode};
use std::path;
use std::io::Read;
use crate::components::cutter::SpritesheetCutter;
use std::ptr::null;

#[derive(Debug)]
pub struct Sprite {
    pub width: i32,
    pub height: i32,
    pub img: Image,
}

pub struct Animation {
    sprites: Vec<Vec<Sprite>>,
    frame_count: i32,
    frame: i32,
}


pub struct SpriteSheet {
    pub sprite_w: i32,
    pub sprite_h: i32,
    pub(crate) image: graphics::Image,
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        Sprite {
            width: self.width,
            height: self.height,
            img: self.img.clone()
        }
    }
}

impl SpritesheetCutter for SpriteSheet {
    fn load_spritesheet_default(ctx: &mut Context, path: &str) -> SpriteSheet {
        let mut image: Image = Image::new(ctx, path).unwrap();
        image.set_filter(FilterMode::Nearest);

        let w = 0;
        let h = 0;

        SpriteSheet {
            sprite_w: w,
            sprite_h: h,
            image,
        }
    }

    fn cut_default(&self, ctx: &mut Context) -> Vec<Vec<Sprite>> {
        self.cut_sized(ctx, 0, 0)
    }

    fn cut_sized(&self, ctx: &mut Context, w: i32, h: i32) -> Vec<Vec<Sprite>> {
        let x_begin = 0;
        let y_begin = 0;
        let x_end = self.image.width() as i32;
        let y_end = self.image.height() as i32;

        self.cut(ctx, x_begin, x_end, y_begin, y_end, w, h)
    }

    fn cut(&self, ctx: &mut Context, x_begin: i32, x_end: i32, y_begin: i32, y_end: i32, w: i32, h: i32) -> Vec<Vec<Sprite>> {
        // let rgba: Vec<u8> = image.to_rgba8(ctx).unwrap();
        let data: Vec<u8> = self.image.to_rgba8(ctx).unwrap();
        let img_width = self.image.width();

        let mut sprites: Vec<Vec<Sprite>> = vec![];

        // since rgba data is stored so, that for every pixel there are 4 values, that means that our begin x is actually begin_x * 4
        // same for y

        let x_tiles = (x_end - x_begin) / w;
        let y_tiles = (y_end - y_begin) / h;


        for j in 0..y_tiles {
            let mut sprite_row: Vec<Sprite> = vec![];

            for i in 0..x_tiles {
                let x_b = (x_begin + i * w) * 4;
                let x_e = (x_begin + i * w + w) * 4;
                let y_b = y_begin + j * h;
                let y_e = y_begin + j * h + h;

                let mut rgba_data: Vec<u8> = vec![];

                for y in y_b..y_e {
                    for x in x_b..x_e {
                        let index: usize = (x + y * img_width as i32 * 4) as usize;
                        rgba_data.push(data[index]);
                    }
                }

                let mut img = Image::from_rgba8(ctx, w as u16, h as u16, &rgba_data).unwrap();
                img.set_filter(FilterMode::Nearest);

                let sprite = Sprite {
                    width: w,
                    height: h,
                    img: img,
                };

                sprite_row.push(sprite);

            };

            sprites.push(sprite_row);
        };

        sprites
    }
}

// pub fn load_spritesheet(ctx: &mut Context, path: &str, w: i32, h: i32) -> SpriteSheet {}
//
// pub fn load_sprite(path: &str) -> Sprite {
//     let mut image: Image = Image::new(ctx, path).unwrap();
//     image.set_filter(FilterMode::Nearest);
//
//     Sprite {
//         width: image.width(),
//         height: image.height(),
//         img: image,
//     }
// }


// pub fn sprite_from_spritesheet(sprite_sheet: SpriteShet, x: i32, y: i32) -> Sprite {}
//
// pub fn sprite_from_spritesheet(sprite_sheet: SpriteSheet, x: i32, y: i32, w: i32, h: i32) -> Sprite {}

// pub fn animation(sh: SpriteSheet) -> Animation {}
