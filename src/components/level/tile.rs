use crate::components::gfx::Sprite;
use crate::components::level::types::TileTypes;
use ggez::graphics::Image;

pub struct Tile {
    pub w: i32,
    pub h: i32,
    pub grid_pos: (i32, i32),
    pub pos: (i32, i32),
    pub sprite: Sprite,
    pub extra_sprite: Option<Sprite>,
    pub tile_type: TileTypes,
}

// this probably won't be needed. Will be part of level...
pub struct TileSet {
    pub sprites: Vec<Vec<Sprite>>,
    pub tile_count: i32,
    pub tile_width: i32,
    pub tile_height: i32,
}

impl Tile {
    pub fn width(&self) -> i32 {
        self.w
    }

    pub fn height(&self) -> i32 {
        self.h
    }

    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    pub fn image(&self) -> &Image {
        &self.sprite.img
    }

    pub fn extra_image(&self) -> &Image {
        let sprite = self.extra_sprite.as_ref().unwrap();
        &sprite.img
    }
}
