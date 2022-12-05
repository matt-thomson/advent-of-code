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
        self.pairs.iter().filter(|pair| pair.overlaps()).count()
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
}
