use std::path::PathBuf;

use structopt::StructOpt;

use crate::command;
use crate::intcode;

#[derive(Debug, StructOpt)]
pub struct Day02 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    target: u32,
}

impl command::Command for Day02 {
    fn part_one(&self) -> u32 {
        output(&intcode::parse(&self.input), 12, 2)
    }

    fn part_two(&self) -> u32 {
        let program = intcode::parse(&self.input);
        let max = program.len().min(100) as u32;

        for noun in 0..max {
            for verb in 0..max {
                if output(&program, noun, verb) == self.target {
                    return noun * 100 + verb;
                }
            }
        }

        unreachable!();
    }
}

fn output(program: &[u32], noun: u32, verb: u32) -> u32 {
    let mut program = program.to_vec();
    program[1] = noun;
    program[2] = verb;

    intcode::run(&mut program);

    program[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::command::Command;

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day02.txt");
        let target = 3100;

        let command = Day02 { input, target };

        assert_eq!(command.part_one(), 3100);
    }

    #[test]
    fn test_part_two() {
        let input = PathBuf::from("fixtures/day02.txt");
        let target = 3100;

        let command = Day02 { input, target };

        assert_eq!(command.part_two(), 412);
    }
}
