mod cpu;
mod instruction;

use std::fs;
use std::path::Path;

use eyre::Result;

use crate::cpu::Cpu;
use crate::instruction::Instruction;

#[derive(Debug)]
pub struct Problem {
    instructions: Vec<Instruction>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let instructions: Result<Vec<_>> = input.lines().map(|line| line.parse()).collect();

        Ok(Self {
            instructions: instructions?,
        })
    }

    pub fn part_one(&self) -> i64 {
        let cpu = Cpu::new(&self.instructions);
        cpu.skip(18).step_by(40).take(6).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one();

        assert_eq!(result, 13140);
    }
}
