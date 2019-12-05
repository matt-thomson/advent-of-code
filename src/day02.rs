use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::command;
use crate::intcode::Intcode;

#[derive(Debug, StructOpt)]
pub struct Day02 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    target: i32,
}

impl command::Command for Day02 {
    fn part_one(&self) -> u32 {
        output(&self.read_program(), 12, 2) as u32
    }

    fn part_two(&self) -> u32 {
        let program = self.read_program();
        let max = program.len().min(100) as i32;

        for noun in 0..max {
            for verb in 0..max {
                if output(&program, noun, verb) == self.target {
                    return (noun * 100 + verb) as u32;
                }
            }
        }

        unreachable!();
    }
}

impl Day02 {
    fn read_program(&self) -> Vec<i32> {
        fs::read_to_string(&self.input)
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect()
    }
}

fn output(program: &[i32], noun: i32, verb: i32) -> i32 {
    let mut program = program.to_vec();
    program[1] = noun;
    program[2] = verb;

    let mut intcode = Intcode::new(program);
    intcode.run(&[]);

    intcode.peek(0)
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
