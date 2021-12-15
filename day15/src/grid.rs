use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Grid {
    risk_levels: Vec<Vec<u32>>,
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let risk_levels = input
            .lines()
            .map(|line| line.bytes().map(|c| (c - b'0') as u32).collect())
            .collect();

        Ok(Self { risk_levels })
    }
}

impl Grid {
    pub fn risk_level(&self, (x, y): (usize, usize)) -> u32 {
        self.risk_levels[y][x]
    }

    pub fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = vec![];

        if x > 0 {
            result.push((x - 1, y));
        }

        if x < self.risk_levels[0].len() - 1 {
            result.push((x + 1, y))
        }

        if y > 0 {
            result.push((x, y - 1));
        }

        if y < self.risk_levels.len() - 1 {
            result.push((x, y + 1));
        }

        result
    }

    pub fn width(&self) -> usize {
        self.risk_levels[0].len()
    }

    pub fn height(&self) -> usize {
        self.risk_levels.len()
    }
}
