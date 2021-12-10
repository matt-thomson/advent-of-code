mod bracket;
mod line;
mod syntax_error;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use line::Line;

#[derive(Debug)]
pub struct Problem {
    lines: Vec<Line>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

        Self { lines }
    }

    pub fn part_one(&self) -> u32 {
        self.lines
            .iter()
            .map(|line| line.syntax_error())
            .filter(|syntax_error| syntax_error.is_corrupted())
            .map(|syntax_error| syntax_error.score())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 26397);
    }
}
