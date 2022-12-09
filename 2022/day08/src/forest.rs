use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Forest(Vec<Vec<u64>>);

impl FromStr for Forest {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            input
                .lines()
                .map(|line| line.chars().map(|c| c as u64 - '0' as u64).collect())
                .collect(),
        ))
    }
}

impl Forest {
    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    fn tree(&self, x: usize, y: usize) -> u64 {
        self.0[y][x]
    }

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        self.is_visible_from_top(x, y)
            || self.is_visible_from_bottom(x, y)
            || self.is_visible_from_left(x, y)
            || self.is_visible_from_right(x, y)
    }

    fn is_visible_from_top(&self, x: usize, y: usize) -> bool {
        (0..y).all(|other_y| self.tree(x, other_y) < self.tree(x, y))
    }

    fn is_visible_from_bottom(&self, x: usize, y: usize) -> bool {
        ((y + 1)..self.height()).all(|other_y| self.tree(x, other_y) < self.tree(x, y))
    }

    fn is_visible_from_left(&self, x: usize, y: usize) -> bool {
        (0..x).all(|other_x| self.tree(other_x, y) < self.tree(x, y))
    }

    fn is_visible_from_right(&self, x: usize, y: usize) -> bool {
        ((x + 1)..self.height()).all(|other_x| self.tree(other_x, y) < self.tree(x, y))
    }
}
