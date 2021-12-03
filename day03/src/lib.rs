use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Problem {
    numbers: Vec<Vec<u32>>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let numbers: Vec<Vec<u32>> = reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();

        Self { numbers }
    }

    pub fn part_one(&self) -> u32 {
        let gamma = (0..self.numbers[0].len())
            .fold(0, |acc, position| acc * 2 + self.most_common(position));

        let epsilon = 2_u32.pow(self.numbers[0].len() as u32) - 1 - gamma;

        gamma * epsilon
    }

    fn most_common(&self, position: usize) -> u32 {
        let count_ones: u32 = self.numbers.iter().map(|row| row[position]).sum();
        (count_ones * 2) / (self.numbers.len() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 198);
    }
}
