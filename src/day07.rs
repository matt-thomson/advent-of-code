use std::fs;
use std::path::PathBuf;

use permutohedron::heap_recursive;
use structopt::StructOpt;

use crate::command;
use crate::intcode::Intcode;

#[derive(Debug, StructOpt)]
pub struct Day07 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl command::Command for Day07 {
    fn part_one(&self) -> u32 {
        let program: Vec<i32> = fs::read_to_string(&self.input)
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let mut phases = [0, 1, 2, 3, 4];
        let mut max = 0;

        heap_recursive(&mut phases, |permutation| {
            let output = run(&program, permutation);
            if output > max {
                max = output;
            }
        });

        max as u32
    }

    fn part_two(&self) -> u32 {
        unimplemented!()
    }
}

fn run(program: &[i32], phases: &[u32]) -> i32 {
    let mut intcodes: Vec<_> = phases
        .iter()
        .map(|_| Intcode::new(program.to_vec()))
        .collect();

    let mut signal = 0;

    for (i, phase) in phases.iter().enumerate() {
        let input = [*phase as i32, signal];

        let output = &intcodes[i].run(&input);
        assert!(output.len() == 1);

        signal = output[0];
    }

    signal
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::command::Command;

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day07a.txt");
        let command = Day07 { input };

        assert_eq!(command.part_one(), 43210);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day07b.txt");
        let command = Day07 { input };

        assert_eq!(command.part_one(), 54321);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day07c.txt");
        let command = Day07 { input };

        assert_eq!(command.part_one(), 65210);
    }
}
