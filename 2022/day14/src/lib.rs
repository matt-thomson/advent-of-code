mod cave;
mod rock;
mod segment;

use std::fs;
use std::path::Path;

use eyre::Result;

use crate::cave::Cave;
use crate::rock::Rock;

#[derive(Debug)]
pub struct Problem {
    rocks: Vec<Rock>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let rocks: Result<Vec<Rock>> = input.lines().map(str::parse).collect();

        Ok(Self { rocks: rocks? })
    }

    pub fn part_one(&self) -> Result<usize> {
        let cave = Cave::new(&self.rocks)?;
        Ok(solve(cave))
    }

    pub fn part_two(&self) -> Result<usize> {
        let mut cave = Cave::new(&self.rocks)?;
        cave.add_floor();

        Ok(solve(cave))
    }
}

fn solve(mut cave: Cave) -> usize {
    let mut count = 0;

    while cave.drop() {
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one().unwrap();

        assert_eq!(result, 24);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_two().unwrap();

        assert_eq!(result, 93);
    }
}
