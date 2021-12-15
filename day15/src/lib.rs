mod grid;

use std::fs;
use std::path::Path;

use grid::Grid;
use pathfinding::prelude::dijkstra_partial;

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
        let (risk_levels, end) = dijkstra_partial(
            &(0, 0),
            |&position| {
                self.grid
                    .neighbours(position)
                    .iter()
                    .map(|&next| (next, self.grid.risk_level(next)))
                    .collect::<Vec<_>>()
            },
            |&(x, y)| x == self.grid.width() - 1 && y == self.grid.height() - 1,
        );

        let (_, risk_level) = risk_levels[&end.unwrap()];

        risk_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 40);
    }
}
