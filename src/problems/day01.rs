use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;

use crate::Problem;

#[derive(Debug, Parser)]
pub struct Day01 {
    #[clap(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day01 {
    type Output = usize;

    fn part_one(&self) -> usize {
        count_increases(&self.depths())
    }

    fn part_two(&self) -> usize {
        let windows: Vec<u32> = self
            .depths()
            .windows(3)
            .map(|window| window.iter().sum())
            .collect();

        count_increases(&windows)
    }
}

impl Day01 {
    fn depths(&self) -> Vec<u32> {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect()
    }
}

fn count_increases(input: &[u32]) -> usize {
    input.windows(2).filter(|pair| pair[1] > pair[0]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Problem;

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day01.txt");
        let problem = Day01 { input };

        assert_eq!(problem.part_one(), 7);
    }

    #[test]
    fn test_part_two() {
        let input = PathBuf::from("fixtures/day01.txt");
        let problem = Day01 { input };

        assert_eq!(problem.part_two(), 5);
    }
}
