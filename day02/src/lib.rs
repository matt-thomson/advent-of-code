mod instruction;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use instruction::Instruction;

#[derive(Debug)]
pub struct Problem {
    instructions: Vec<Instruction>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let instructions = reader
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

        Self { instructions }
    }

    pub fn part_one(&self) -> u32 {
        let (position, depth) = self
            .instructions
            .iter()
            .fold((0, 0), |position, instruction| instruction.step(position));

        position * depth
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 150);
    }
}
