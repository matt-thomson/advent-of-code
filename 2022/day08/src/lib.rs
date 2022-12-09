mod forest;

use std::fs;
use std::path::Path;

use eyre::Result;
use forest::Forest;

#[derive(Debug)]
pub struct Problem {
    forest: Forest,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let forest = input.parse()?;

        Ok(Self { forest })
    }

    pub fn part_one(&self) -> usize {
        (0..self.forest.width())
            .map(|x| {
                (0..self.forest.height())
                    .filter(|y| self.forest.is_visible(x, *y))
                    .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one();

        assert_eq!(result, 21);
    }
}
