mod board;
mod game;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use board::Board;
use game::{play, Result};

#[derive(Debug)]
pub struct Problem {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let lines: Vec<_> = BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        let numbers = lines[0]
            .split(',')
            .map(|number| number.parse().unwrap())
            .collect();

        let boards = lines[1..].chunks(5).map(Board::new).collect();

        Self { numbers, boards }
    }

    pub fn part_one(&self) -> u32 {
        self.run(|results| results.into_iter().min_by_key(|result| result.steps))
    }

    pub fn part_two(&self) -> u32 {
        self.run(|results| results.into_iter().max_by_key(|result| result.steps))
    }

    fn run<F>(&self, selector: F) -> u32
    where
        F: Fn(Vec<Result>) -> Option<Result>,
    {
        let results: Vec<Result> = self
            .boards
            .iter()
            .map(|board| play(board, &self.numbers))
            .collect();

        let result = selector(results).unwrap();

        result.score * self.numbers[result.steps]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 4512);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 1924);
    }
}
