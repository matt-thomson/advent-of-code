use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub enum FoldDirection {
    Left,
    Up,
}

#[derive(Debug)]
pub struct Fold {
    direction: FoldDirection,
    position: usize,
}

impl FromStr for Fold {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let direction = match input.chars().nth(11).unwrap() {
            'x' => FoldDirection::Left,
            'y' => FoldDirection::Up,
            _ => panic!("unknown fold direction"),
        };

        let (_, position_str) = input.split_once('=').unwrap();
        let position = position_str.parse()?;

        Ok(Self {
            direction,
            position,
        })
    }
}

impl Fold {
    pub fn image(&self, &(x, y): &(usize, usize)) -> (usize, usize) {
        match self.direction {
            FoldDirection::Left => {
                if x < self.position {
                    (x, y)
                } else {
                    (x - 2 * (x - self.position), y)
                }
            }
            FoldDirection::Up => {
                if y < self.position {
                    (x, y)
                } else {
                    (x, y - 2 * (y - self.position))
                }
            }
        }
    }
}
