use std::collections::HashMap;

use super::tile::Tile;

type Position = (i64, i64);

pub struct Screen {
    tiles: HashMap<Position, Tile>,
    score: usize,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            tiles: HashMap::new(),
            score: 0,
        }
    }

    pub fn update(&mut self, output: &[i64]) {
        for chunk in output.chunks(3) {
            let position = (chunk[0], chunk[1]);

            if position == (-1, 0) {
                self.score = chunk[2] as usize;
            } else {
                let tile = Tile::from(chunk[2]);
                self.tiles.insert(position, tile);
            }
        }
    }

    pub fn num_blocks(&self) -> usize {
        self.tiles
            .iter()
            .filter(|(_, tile)| **tile == Tile::Block)
            .count()
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn find(&self, tile: Tile) -> Position {
        let (position, _) = self.tiles.iter().find(|(_, t)| **t == tile).unwrap();

        *position
    }
}
