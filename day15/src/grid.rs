use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Grid {
    risk_levels: Vec<Vec<usize>>,
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let risk_levels = input
            .lines()
            .map(|line| line.bytes().map(|c| (c - b'0') as usize).collect())
            .collect();

        Ok(Self { risk_levels })
    }
}

impl Grid {
    pub fn risk_level(&self, (x, y): (usize, usize)) -> usize {
        let x_tile = x / self.risk_levels[0].len();
        let y_tile = y / self.risk_levels.len();

        let raw_pos = self.risk_levels[y % self.risk_levels.len()][x % self.risk_levels[0].len()];

        (raw_pos + x_tile + y_tile - 1) % 9 + 1
    }

    pub fn neighbours(&self, (x, y): (usize, usize), repeats: usize) -> Vec<(usize, usize)> {
        let mut result = vec![];

        if x > 0 {
            result.push((x - 1, y));
        }

        if x < (self.risk_levels[0].len() * repeats) - 1 {
            result.push((x + 1, y));
        }

        if y > 0 {
            result.push((x, y - 1));
        }

        if y < (self.risk_levels.len() * repeats) - 1 {
            result.push((x, y + 1));
        }

        result
    }

    pub fn width(&self, repeats: usize) -> usize {
        self.risk_levels[0].len() * repeats
    }

    pub fn height(&self, repeats: usize) -> usize {
        self.risk_levels.len() * repeats
    }
}
