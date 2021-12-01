use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Day01 {
    #[clap(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let problem = Day01::parse();

    println!("Part 1: {}", problem.part_one());
    println!("Part 2: {}", problem.part_two());
}

impl Day01 {
    fn part_one(&self) -> usize {
        self.count_increases(2)
    }

    fn part_two(&self) -> usize {
        self.count_increases(4)
    }

    fn count_increases(&self, window_size: usize) -> usize {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        let depths: Vec<u32> = reader
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

        depths
            .windows(window_size)
            .filter(|pair| pair.last() > pair.first())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
