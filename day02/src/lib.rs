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
        let (position, depth) =
            self.run(|position, instruction| instruction.step_part_one(position));

        position * depth
    }

    pub fn part_two(&self) -> u32 {
        let (position, depth, _aim) =
            self.run(|position, instruction| instruction.step_part_two(position));

        position * depth
    }

    fn run<T, F>(&self, reducer: F) -> T
    where
        T: Default,
        F: FnMut(T, &Instruction) -> T,
    {
        self.instructions.iter().fold(T::default(), reducer)
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

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 900);
    }
}
