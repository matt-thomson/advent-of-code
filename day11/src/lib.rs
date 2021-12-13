mod grid;

use std::fs;
use std::path::Path;

use grid::Grid;

#[derive(Debug)]
pub struct Problem {
    grid: Grid,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let grid: Grid = fs::read_to_string(&path).unwrap().parse().unwrap();

        Self { grid }
    }

    pub fn part_one(&self) -> usize {
        let mut grid = self.grid.clone();
        (0..100).map(|_| grid.step()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 1656);
    }
}
