use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Intcode;
use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day02 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    target: i64,
}

impl Problem for Day02 {
    type Output = i64;

    fn part_one(&self) -> i64 {
        output(&self.read_program(), 12, 2)
    }

    fn part_two(&self) -> i64 {
        let program = self.read_program();
        let max = program.len().min(100) as i64;

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

impl Day02 {
    fn read_program(&self) -> Vec<i64> {
        fs::read_to_string(&self.input)
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect()
    }
}

fn output(program: &[i64], noun: i64, verb: i64) -> i64 {
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

    use crate::problem::Problem;

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day02.txt");
        let target = 3100;

        let problem = Day02 { input, target };

        assert_eq!(problem.part_one(), 3100);
    }

    #[test]
    fn test_part_two() {
        let input = PathBuf::from("fixtures/day02.txt");
        let target = 3100;

        let problem = Day02 { input, target };

        assert_eq!(problem.part_two(), 412);
    }
}
