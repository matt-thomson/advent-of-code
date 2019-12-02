use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::command;

#[derive(Debug, StructOpt)]
pub struct Day02 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl command::Command for Day02 {
    fn part_one(&self) -> u32 {
        let input = fs::read_to_string(&self.input).unwrap();
        let mut program: Vec<u32> = input
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        program[1] = 12;
        program[2] = 2;

        run(&mut program);

        program[0]
    }

    fn part_two(&self) -> u32 {
        unimplemented!()
    }
}

fn run(program: &mut [u32]) {
    for counter in (0..).step_by(4) {
        match program[counter] {
            1 => {
                let first = program[counter + 1] as usize;
                let second = program[counter + 2] as usize;
                let location = program[counter + 3] as usize;
                program[location] = program[first] + program[second];
            }
            2 => {
                let first = program[counter + 1] as usize;
                let second = program[counter + 2] as usize;
                let location = program[counter + 3] as usize;
                program[location] = program[first] * program[second];
            }
            99 => break,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::command::Command;

    #[test]
    fn test_run_one() {
        let mut program = vec![1, 0, 0, 0, 99];
        run(&mut program);

        assert_eq!(program, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_run_two() {
        let mut program = vec![2, 3, 0, 3, 99];
        run(&mut program);

        assert_eq!(program, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_run_three() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        run(&mut program);

        assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_run_four() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut program);

        assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_run_five() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run(&mut program);

        assert_eq!(program, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day02/input.txt");
        let command = Day02 { input };

        assert_eq!(command.part_one(), 3100);
    }
}
