use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Problem {
    depths: Vec<u32>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let depths: Vec<u32> = reader
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

        Self { depths }
    }

    pub fn part_one(&self) -> usize {
        self.count_increases(2)
    }

    pub fn part_two(&self) -> usize {
        self.count_increases(4)
    }

    fn count_increases(&self, window_size: usize) -> usize {
        self.depths
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
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 7);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 5);
    }
}
