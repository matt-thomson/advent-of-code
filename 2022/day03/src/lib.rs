mod item;
mod rucksack;

use std::fs;
use std::path::Path;

use eyre::Result;
use rucksack::Rucksack;

#[derive(Debug)]
pub struct Problem {
    rucksacks: Vec<Rucksack>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let rucksacks: Result<Vec<_>> = fs::read_to_string(path)?
            .lines()
            .map(|line| Ok(line.trim().parse()?))
            .collect();

        Ok(Self {
            rucksacks: rucksacks?,
        })
    }

    pub fn part_one(&self) -> Result<u64> {
        self.rucksacks
            .iter()
            .map(|rucksack| rucksack.duplicate()?.priority())
            .try_fold(0, |acc, priority| priority.map(|priority| acc + priority))
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one().unwrap();

        assert_eq!(result, 157);
    }
}
