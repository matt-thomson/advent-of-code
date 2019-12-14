#[derive(Debug, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    pub fn from(value: i64) -> Self {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            x => panic!("unknown tile {}", x),
        }
    }
}
