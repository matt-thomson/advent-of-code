use std::fs;
use std::path::Path;

use eyre::{eyre, Result};

#[derive(Debug)]
pub struct Problem {
    elves: Vec<Vec<u64>>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let lines: Vec<_> = content.lines().collect();

        let elves: Result<Vec<Vec<u64>>> = lines
            .split(|line| line.trim().is_empty())
            .map(|lines| {
                lines
                    .iter()
                    .map(|line| Ok(line.trim().parse::<u64>()?))
                    .collect()
            })
            .collect();

        Ok(Self { elves: elves? })
    }

    pub fn part_one(&self) -> Result<u64> {
        self.solve(1)
    }

    pub fn part_two(&self) -> Result<u64> {
        self.solve(3)
    }

    fn solve(&self, count: usize) -> Result<u64> {
        let mut calories: Vec<_> = self.elves.iter().map(|elf| elf.iter().sum()).collect();
        calories.sort_unstable();
        calories.reverse();

        Ok(calories
            .get(0..count)
            .ok_or_else(|| eyre!("not enough elves"))?
            .iter()
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one().unwrap();

        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_two().unwrap();

        assert_eq!(result, 45000);
    }
}
