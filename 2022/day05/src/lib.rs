mod stacks;
mod step;

use std::fs;
use std::path::Path;

use eyre::{eyre, Result};

use crate::stacks::Stacks;
use crate::step::Step;

#[derive(Debug)]
pub struct Problem {
    stacks: Stacks,
    steps: Vec<Step>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let (stacks, steps) = input
            .split_once("\n\n")
            .ok_or_else(|| eyre!("couldn't split into stacks and moves"))?;

        Ok(Self {
            stacks: stacks.parse()?,
            steps: steps
                .lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<_>>>()?,
        })
    }

    pub fn part_one(&self) -> Result<String> {
        let mut stacks = self.stacks.clone();

        for step in self.steps.iter() {
            stacks.apply(step)?;
        }

        stacks.tops()
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one().unwrap();

        assert_eq!(result, "CMZ");
    }
}
