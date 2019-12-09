use std::fs;
use std::path::PathBuf;

use permutohedron::heap_recursive;
use structopt::StructOpt;

use crate::intcode::Intcode;
use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day07 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day07 {
    type Output = i64;

    fn part_one(&self) -> i64 {
        self.solve(&mut [0, 1, 2, 3, 4])
    }

    fn part_two(&self) -> i64 {
        self.solve(&mut [5, 6, 7, 8, 9])
    }
}

impl Day07 {
    fn solve(&self, phases: &mut [i64; 5]) -> i64 {
        let program: Vec<i64> = fs::read_to_string(&self.input)
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

        max
    }
}

fn run(program: &[i64], phases: &[i64]) -> i64 {
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

fn init(program: &[i64], phase: i64) -> Intcode {
    let mut intcode = Intcode::new(program.to_vec());
    let output = intcode.run(&[phase]);

    assert!(output.is_empty());

    intcode
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::problem::Problem;

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day07a.txt");
        let problem = Day07 { input };

        assert_eq!(problem.part_one(), 43210);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day07b.txt");
        let problem = Day07 { input };

        assert_eq!(problem.part_one(), 54321);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day07c.txt");
        let problem = Day07 { input };

        assert_eq!(problem.part_one(), 65210);
    }

    #[test]
    fn test_part_two_d() {
        let input = PathBuf::from("fixtures/day07d.txt");
        let problem = Day07 { input };

        assert_eq!(problem.part_two(), 139_629_729);
    }

    #[test]
    fn test_part_two_e() {
        let input = PathBuf::from("fixtures/day07e.txt");
        let problem = Day07 { input };

        assert_eq!(problem.part_two(), 18216);
    }
}
