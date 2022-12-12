mod cpu;
mod instruction;

use std::fs;
use std::path::Path;

use eyre::Result;
use itertools::Itertools;

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

        cpu.enumerate()
            .skip(19)
            .step_by(40)
            .take(6)
            .map(|(cycle, x)| (cycle as i64 + 1) * x)
            .sum()
    }

    pub fn part_two(&self) -> String {
        let cpu = Cpu::new(&self.instructions);

        cpu.chunks(40)
            .into_iter()
            .map(|line| draw_line(line))
            .join("\n")
    }
}

fn draw_line<I: Iterator<Item = i64>>(line: I) -> String {
    line.enumerate()
        .map(|(position, x)| {
            if x.abs_diff(position as i64) <= 1 {
                '█'
            } else {
                ' '
            }
        })
        .collect()
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
