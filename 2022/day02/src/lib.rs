use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Problem {
    rounds: Vec<String>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let rounds = fs::read_to_string(path)?
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        Ok(Self { rounds })
    }

    pub fn part_one(&self) -> Result<u64> {
        self.rounds
            .iter()
            .map(|round| score(round))
            .try_fold(0, |acc, score| score.map(|score| acc + score))
    }
}

fn score(round: &str) -> Result<u64> {
    match round {
        "A X" => Ok(4),
        "A Y" => Ok(8),
        "A Z" => Ok(3),
        "B X" => Ok(1),
        "B Y" => Ok(5),
        "B Z" => Ok(9),
        "C X" => Ok(7),
        "C Y" => Ok(2),
        "C Z" => Ok(6),
        _ => Err(anyhow!("invalid round {round}")),
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one().unwrap();

        assert_eq!(result, 15);
    }
}
