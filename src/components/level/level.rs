use crate::components::level::tile::Tile;
use crate::components::gfx::{Sprite, SpriteSheet};
use crate::components::level::types::TileTypes;
use ggez::{Context, graphics};
use crate::components::cutter::SpritesheetCutter;
use ggez::graphics::{DrawParam, Color, Image};
use ggez::mint::{Point2, Vector2};
use rand;
use rand::Rng;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::graphics::spritebatch::SpriteBatch;

pub struct Level {
    pub tiles: Vec<Vec<Tile>>,
    pub width: i32,
    pub height: i32,
    pub x_offset: f32,
    pub y_offset: f32,
}

impl Level {
    pub fn new(ctx: &mut Context, w: i32, h: i32) -> Level {
        let mut tiles: Vec<Vec<Tile>> = vec![];

        let sh = SpriteSheet::load_spritesheet_default(ctx, "/tiles/floortiles.png");
        let sprites = sh.cut(ctx, 0, 224, 0, 96, 32, 32);

        for x in 0..w {
            let mut tiles_row: Vec<Tile> = vec![];
            for y in 0..h {
                tiles_row.push(Self::random_tile(&sprites, (x, y)));
                // tiles.push(Self::random_tile(&sprites, (x, y)));
            }

            tiles.push(tiles_row);
        }

        Level {
            tiles: tiles,
            width: w,
            height: h,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }

    pub fn from_image(ctx: &mut Context, path: &str) -> Level {
        let mut tiles: Vec<Vec<Tile>> = vec![];

        let level_image: Image = Image::new(ctx, path).unwrap();
        let data = level_image.to_rgba8(ctx).unwrap();
        let length = level_image.width() * level_image.height();

        // load tile images
        let sh = SpriteSheet::load_spritesheet_default(ctx, "/tiles/floortiles.png");
        let floor_tiles = sh.cut(ctx, 0, 224, 0, 96, 32, 32);

        let sh = SpriteSheet::load_spritesheet_default(ctx, "/tiles/walltiles.png");
        let mut walls_top = sh.cut(ctx, 0, 256, 0, 32, 32, 32);
        let mut walls_front = sh.cut(ctx, 0, 256, 32, 56, 32, 24);

        let mut wall_tiles = vec![walls_top[0].to_vec(), walls_front[0].to_vec()];

        for y in 0..level_image.height() as i32 {
            let mut tiles_row: Vec<Tile> = vec![];

            for x in 0..level_image.width() as i32 {
                let t_type: TileTypes;

                let index = ((x + y * level_image.width() as i32) * 4) as usize;

                // let index = (x * 4 + y * level_image.width() as i32) as usize;

                let r = data[index];
                let g = data[index + 1];
                let b = data[index + 2];

                match (r, g, b) {
                    (255, 255, 255) => {
                        let tile = Self::create_tile(&floor_tiles, (x, y), TileTypes::Floor, &mut tiles);
                        tiles_row.push(tile);
                    }

                    (0, 0, 0) => {
                        let tile = Self::create_tile(&floor_tiles, (x, y), TileTypes::Hole, &mut tiles);
                        tiles_row.push(tile);
                    }

                    (255, 0, 0) => {
                        let tile = Self::create_tile(&wall_tiles, (x, y), TileTypes::Wall, &mut tiles);
                        tiles_row.push(tile);
                    }

                    _ => {
                        let tile = Self::create_tile(&floor_tiles, (x, y), TileTypes::None, &mut tiles);
                        tiles_row.push(tile);
                    }
                }
            }

            tiles.push(tiles_row);
        }

        Level {
            tiles,
            width: level_image.width() as i32,
            height: level_image.height() as i32,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }

    pub fn create_tile(sprites: &Vec<Vec<Sprite>>, pos: (i32, i32), tt: TileTypes, tiles: &mut Vec<Vec<Tile>>) -> Tile {
        let mut rng = rand::thread_rng();

        let (x, y) = pos;

        match tt {
            TileTypes::Floor => {
                let index = rng.gen_range(0, 4);
                let sprite = sprites[0][index].clone();

                Tile {
                    w: 32,
                    h: 32,
                    grid_pos: (x, y),
                    pos: (x * 32, y * 32),
                    sprite: sprite,
                    extra_sprite: None,
                    tile_type: tt,
                }
            }

            TileTypes::Hole => {
                // used to set tile image to transparent part of spritesheet
                // extra hole tile image is used only where hole is on the edge of floortile, that is if the tile above is not Hole tile
                let mut y_index = 0;

                // check what is the tile above current
                if tiles.len() >= y as usize {
                    if tiles[(y - 1) as usize].len() >= x as usize {
                        if x == 43 {
                            println!("X: {} | Y: {}", x, y);
                        }
                        let t = Self::get_tile_type(tiles, (x, y - 1));

                        match t {
                            TileTypes::Hole => { y_index = 2; }
                            _ => { () }
                        }
                    }
                }

                let sprite = sprites[y_index as usize][4].clone();

                Tile {
                    w: 32,
                    h: 32,
                    grid_pos: (x, y),
                    pos: (x * 32, y * 32),
                    sprite: sprite,
                    extra_sprite: None,
                    tile_type: tt,
                }
            }

            TileTypes::Wall => {
                let index = rng.gen_range(0, 8);
                let sprite = sprites[0][index].clone();
                let extra_sprite = sprites[1][index].clone();

                let tile_type = TileTypes::WallFront;
                // if tile above is wall, that tile is not front wall!
                if tiles.len() >= y as usize && y > 0 {
                    if tiles[(y - 1) as usize].len() >= x as usize {
                        let mut tile = &mut tiles[(y - 1) as usize][x as usize];
                        match tile.tile_type {
                            TileTypes::WallFront => {
                                tile.tile_type = TileTypes::Wall;
                            }
                            _ => { () }
                        }
                    }
                }

                Tile {
                    w: 32,
                    h: 56,
                    grid_pos: (x, y),
                    pos: (x * 32, y * 32 - 24), // offset because it is 24px taller
                    sprite: sprite,
                    extra_sprite: Some(extra_sprite),
                    tile_type: tile_type,
                }
            }
            _ => {
                Tile {
                    w: 0,
                    h: 0,
                    grid_pos: (0, 0),
                    pos,
                    sprite: sprites[0][0].clone(),
                    extra_sprite: None,
                    tile_type: TileTypes::None,
                }
            }
        }
    }

    pub fn get_tile_type(tiles: &Vec<Vec<Tile>>, pos: (i32, i32)) -> &TileTypes {
        let x = pos.0 as usize;
        let y = pos.1 as usize;

        if tiles.len() > y {
            if tiles[y].len() > x {
                return &tiles[y][x].tile_type;
            }
        }

        &TileTypes::None
    }

    pub fn random_tile(sprites: &Vec<Vec<Sprite>>, pos: (i32, i32)) -> Tile {
        let mut rng = rand::thread_rng();

        let t = TileTypes::next_random();
        let s: Sprite;

        match t {
            _ => {
                let index = rng.gen_range(0, 4) as usize;
                s = sprites[0][index].clone();

                let mut tile = Tile {
                    w: s.width,
                    h: s.height,
                    grid_pos: (pos.0, pos.1),
                    pos: (pos.0 * 32, pos.1 * 32),
                    sprite: s,
                    extra_sprite: None,
                    tile_type: TileTypes::Floor,
                };

                tile
            }
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn tiles_mut(&mut self) -> &mut Vec<Vec<Tile>> {
        &mut self.tiles
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let amount = 2.5;

        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.x_offset += amount;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.x_offset -= amount;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.y_offset += amount;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.y_offset -= amount;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let scale = 2.0;
        let mut spritebatch: SpriteBatch;


        let param = DrawParam::new().dest(Point2 { x: self.x_offset, y: self.y_offset });

        for tile_row in &self.tiles {
            for tile in tile_row {
                let screen = graphics::drawable_size(ctx);

                let tile_x: f32 = tile.pos.0 as f32 + self.x_offset;
                let tile_y: f32 = tile.pos.1 as f32 + self.y_offset;

                if tile_x < 0.0 || tile_x > screen.0 || tile_y < 0.0 || tile_y > screen.1 {
                    continue;
                }

                match tile.tile_type {
                    TileTypes::None => { () }

                    TileTypes::WallFront => {
                        let pos = (tile.pos.0 as f32, tile.pos.1 as f32);
                        let img = tile.image();
                        let front_img = tile.extra_image();
                        let dest = Point2 { x: pos.0 * scale, y: pos.1 * scale };


                        let mut params = DrawParam::new();
                        params.dest = dest;
                        params.scale = Vector2 { x: scale, y: scale };

                        spritebatch = SpriteBatch::new(img.clone());
                        spritebatch.add(param);

                        graphics::draw(ctx, &spritebatch, params);

                        let front_dest = Point2 { x: pos.0 * scale, y: (pos.1 + 32.0) * scale };
                        let mut front_params = DrawParam::new();
                        front_params.dest = front_dest;
                        front_params.scale = Vector2 { x: scale, y: scale };

                        spritebatch.set_image(front_img.clone());

                        graphics::draw(ctx, &spritebatch, front_params);
                        // graphics::draw(ctx, front_img, front_params);

                        spritebatch.clear();
                    }

                    _ => {
                        // let pos = (tile.pos.0 as f32, tile.pos.1 as f32);
                        let pos = (tile.pos.0 as f32, tile.pos.1 as f32);
                        let img = tile.image();
                        let dest = Point2 { x: pos.0 * scale, y: pos.1 * scale };

                        spritebatch = SpriteBatch::new(img.clone());
                        spritebatch.add(param);

                        let mut params = DrawParam::new();
                        params.dest = dest;
                        params.scale = Vector2 { x: scale, y: scale };

                        graphics::draw(ctx, &spritebatch, params);
                        spritebatch.clear();
                    }
                }
            }
        }
    }
}