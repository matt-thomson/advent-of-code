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

    pub fn part_one(&self) -> u32 {
        self.grid
            .low_points()
            .iter()
            .map(|&position| self.grid.height(position) + 1)
            .sum()
    }

    pub fn part_two(&self) -> usize {
        let mut basin_sizes: Vec<_> = self
            .grid
            .low_points()
            .iter()
            .map(|&position| self.grid.basin_size(position))
            .collect();

        basin_sizes.sort_unstable();
        basin_sizes.reverse();

        basin_sizes.iter().take(3).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 15);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 1134);
    }
}
