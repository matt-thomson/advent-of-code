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
        self.solve(&mut [0, 1, 2, 3, 4])
    }

    fn part_two(&self) -> u32 {
        self.solve(&mut [5, 6, 7, 8, 9])
    }
}

impl Day07 {
    fn solve(&self, phases: &mut [u32; 5]) -> u32 {
        let program: Vec<i32> = fs::read_to_string(&self.input)
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let mut max = 0;

        heap_recursive(phases, |permutation| {
            let output = run(&program, permutation);
            if output > max {
                max = output;
            }
        });

        max as u32
    }
}

fn run(program: &[i32], phases: &[u32]) -> i32 {
    let mut intcodes: Vec<_> = phases.iter().map(|phase| init(&program, *phase)).collect();

    let mut signal = 0;

    while !intcodes[4].is_halted() {
        for intcode in &mut intcodes {
            let output = intcode.run(&[signal]);
            assert!(output.len() == 1);

            signal = output[0];
        }
    }

    signal
}

fn init(program: &[i32], phase: u32) -> Intcode {
    let mut intcode = Intcode::new(program.to_vec());
    let output = intcode.run(&[phase as i32]);

    assert!(output.is_empty());

    intcode
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

    #[test]
    fn test_part_two_d() {
        let input = PathBuf::from("fixtures/day07d.txt");
        let command = Day07 { input };

        assert_eq!(command.part_two(), 139_629_729);
    }

    #[test]
    fn test_part_two_e() {
        let input = PathBuf::from("fixtures/day07e.txt");
        let command = Day07 { input };

        assert_eq!(command.part_two(), 18216);
    }
}
