#[derive(Debug)]
pub enum TileTypes {
    Floor,
    Wall,
    WallFront,
    Hole,
    None,

}

impl TileTypes {
    pub fn next_random() -> Self {
        Self::Floor
    }
}