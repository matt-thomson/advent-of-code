mod snailfish_number;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use snailfish_number::SnailfishNumber;

#[derive(Debug)]
pub struct Problem {
    snailfish_numbers: Vec<SnailfishNumber>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let snailfish_numbers = reader
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

        Self { snailfish_numbers }
    }

    pub fn part_one(&self) -> u32 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 4140);
    }
}
