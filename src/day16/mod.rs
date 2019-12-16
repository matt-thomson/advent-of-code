mod pattern;

use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

use pattern::Pattern;

const NUM_PHASES: usize = 100;

#[derive(Debug, StructOpt)]
pub struct Day16 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day16 {
    type Output = i32;

    fn part_one(&self) -> i32 {
        let mut elements = parse(&fs::read_to_string(&self.input).unwrap().trim());

        for _ in 0..NUM_PHASES {
            elements = fft(&elements);
        }

        elements[0..8].iter().fold(0, |acc, x| acc * 10 + x)
    }

    fn part_two(&self) -> i32 {
        unimplemented!();
    }
}

fn parse(input: &str) -> Vec<i32> {
    input.bytes().map(|c| (c as i32) - 48).collect()
}

fn fft(input: &[i32]) -> Vec<i32> {
    (0..input.len()).map(|i| element(&input, i)).collect()
}

fn element(input: &[i32], position: usize) -> i32 {
    let sum: i32 = input
        .iter()
        .zip(Pattern::new(position))
        .map(|(x, y)| (x * y))
        .sum();

    sum.abs() % 10
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

        assert_eq!(problem.part_one(), 24176176);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day16b.txt");
        let problem = Day16 { input };

        assert_eq!(problem.part_one(), 73745418);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day16c.txt");
        let problem = Day16 { input };

        assert_eq!(problem.part_one(), 52432133);
    }
}
