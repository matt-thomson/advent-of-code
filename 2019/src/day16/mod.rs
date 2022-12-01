mod pattern;

use std::fs;
use std::iter;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

use pattern::Pattern;

const NUM_PHASES: usize = 100;
const NUM_REPEATS: usize = 10000;

#[derive(Debug, StructOpt)]
pub struct Day16 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day16 {
    type Output = i32;

    fn part_one(&self) -> i32 {
        let mut input = parse(fs::read_to_string(&self.input).unwrap().trim());

        for _ in 0..NUM_PHASES {
            input = fft(&input);
        }

        read(&input, 8)
    }

    fn part_two(&self) -> i32 {
        let input = parse(fs::read_to_string(&self.input).unwrap().trim());
        let offset = read(&input, 7) as usize;

        assert!(offset > input.len() * NUM_REPEATS / 2);

        let mut fast_input: Vec<_> = iter::repeat(input)
            .take(NUM_REPEATS)
            .flatten()
            .skip(offset)
            .collect();

        for _ in 0..NUM_PHASES {
            fast_input = fast_fft(&fast_input);
        }

        read(&fast_input, 8)
    }
}

fn parse(input: &str) -> Vec<i32> {
    input.bytes().map(|c| (c as i32) - 48).collect()
}

fn fft(input: &[i32]) -> Vec<i32> {
    (0..input.len()).map(|i| element(input, i)).collect()
}

fn fast_fft(input: &[i32]) -> Vec<i32> {
    let mut output = vec![0; input.len()];
    let mut total = 0;

    for i in (0..(input.len())).rev() {
        total = (total + input[i]) % 10;
        output[i] = total;
    }

    output
}

fn element(input: &[i32], position: usize) -> i32 {
    let sum: i32 = input
        .iter()
        .zip(Pattern::new(position))
        .map(|(x, y)| (x * y))
        .sum();

    sum.abs() % 10
}

fn read(input: &[i32], digits: usize) -> i32 {
    input[0..digits].iter().fold(0, |acc, x| acc * 10 + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("15243"), vec![1, 5, 2, 4, 3]);
    }

    #[test]
    fn test_fft() {
        assert_eq!(fft(&[1, 2, 3, 4, 5, 6, 7, 8]), vec![4, 8, 2, 2, 6, 1, 5, 8]);
        assert_eq!(fft(&[4, 8, 2, 2, 6, 1, 5, 8]), vec![3, 4, 0, 4, 0, 4, 3, 8]);
        assert_eq!(fft(&[3, 4, 0, 4, 0, 4, 3, 8]), vec![0, 3, 4, 1, 5, 5, 1, 8]);
        assert_eq!(fft(&[0, 3, 4, 1, 5, 5, 1, 8]), vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day16a.txt");
        let problem = Day16 { input };

        assert_eq!(problem.part_one(), 24_176_176);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day16b.txt");
        let problem = Day16 { input };

        assert_eq!(problem.part_one(), 73_745_418);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day16c.txt");
        let problem = Day16 { input };

        assert_eq!(problem.part_one(), 52_432_133);
    }

    #[test]
    fn test_part_two_d() {
        let input = PathBuf::from("fixtures/day16d.txt");
        let problem = Day16 { input };

        assert_eq!(problem.part_two(), 84_462_026);
    }

    #[test]
    fn test_part_two_e() {
        let input = PathBuf::from("fixtures/day16e.txt");
        let problem = Day16 { input };

        assert_eq!(problem.part_two(), 78_725_270);
    }

    #[test]
    fn test_part_two_f() {
        let input = PathBuf::from("fixtures/day16f.txt");
        let problem = Day16 { input };

        assert_eq!(problem.part_two(), 53_553_731);
    }
}
