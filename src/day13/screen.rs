use std::collections::HashSet;

use super::tile::Tile;

type Position = (i64, i64);

pub struct Screen {
    score: usize,
    blocks: HashSet<Position>,
    ball: Position,
    paddle: Position,
}

impl Screen {
    pub fn new(output: &[i64]) -> Self {
        let mut score = 0;
        let mut blocks = HashSet::new();
        let mut ball = None;
        let mut paddle = None;

        for chunk in output.chunks(3) {
            let position = (chunk[0], chunk[1]);

            if position == (-1, 0) {
                score = chunk[2] as usize;
            } else {
                let tile = Tile::from(chunk[2]);

                match tile {
                    Tile::Empty => (),
                    Tile::Wall => (),
                    Tile::Block => {
                        blocks.insert(position);
                        ()
                    }
                    Tile::Ball => ball = Some(position),
                    Tile::Paddle => paddle = Some(position),
                }
            }
        }

        Screen {
            score,
            blocks,
            ball: ball.unwrap(),
            paddle: paddle.unwrap(),
        }
    }

    pub fn update(&mut self, output: &[i64]) {
        for chunk in output.chunks(3) {
            let position = (chunk[0], chunk[1]);

            if position == (-1, 0) {
                self.score = chunk[2] as usize;
            } else {
                let tile = Tile::from(chunk[2]);

                match tile {
                    Tile::Empty => {
                        self.blocks.remove(&position);
                        ()
                    }
                    Tile::Wall => unreachable!(),
                    Tile::Block => unreachable!(),
                    Tile::Ball => self.ball = position,
                    Tile::Paddle => self.paddle = position,
                }
            }
        }
    }

    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn ball(&self) -> Position {
        self.ball
    }

    pub fn paddle(&self) -> Position {
        self.paddle
    }
}
