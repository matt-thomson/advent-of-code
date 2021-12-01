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
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        let depths: Vec<u32> = reader
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

        depths.windows(2).filter(|pair| pair[1] > pair[0]).count()
    }

    fn part_two(&self) -> usize {
        unimplemented!()
    }
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
}
