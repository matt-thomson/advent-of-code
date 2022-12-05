mod pair;

use std::fs;
use std::path::Path;

use eyre::Result;

use crate::pair::Pair;

#[derive(Debug)]
pub struct Problem {
    pairs: Vec<Pair>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let pairs: Result<Vec<Pair>> = fs::read_to_string(path)?
            .lines()
            .map(|line| line.parse())
            .collect();

        Ok(Self { pairs: pairs? })
    }

    pub fn part_one(&self) -> usize {
        self.solve(|pair| pair.fully_overlaps())
    }

    pub fn part_two(&self) -> usize {
        self.solve(|pair| pair.partly_overlaps())
    }

    fn solve<P: Fn(&&Pair) -> bool>(&self, predicate: P) -> usize {
        self.pairs.iter().filter(predicate).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_two();

        assert_eq!(result, 4);
    }
}
