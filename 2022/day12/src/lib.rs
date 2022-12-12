mod heightmap;

use std::fs;
use std::path::Path;

use eyre::{eyre, Result};
use pathfinding::prelude::bfs;

use crate::heightmap::Heightmap;

#[derive(Debug)]
pub struct Problem {
    heightmap: Heightmap,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let heightmap = input.parse()?;

        Ok(Self { heightmap })
    }

    pub fn part_one(&self) -> Result<usize> {
        bfs(
            self.heightmap.end(),
            |position| self.heightmap.neighbours(*position),
            |position| position == self.heightmap.start(),
        )
        .ok_or_else(|| eyre!("couldn't find path"))
        .map(|path| path.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one().unwrap();

        assert_eq!(result, 31);
    }
}
