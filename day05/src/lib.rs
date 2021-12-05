mod line;

use std::collections::HashMap;
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

        let lines: Vec<Line> = reader
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

        Self { lines }
    }

    pub fn part_one(&self) -> usize {
        let mut vents = HashMap::new();

        self.lines
            .iter()
            .flat_map(|line| line.points())
            .for_each(|point| *vents.entry(point).or_insert(0) += 1);

        vents.iter().filter(|(_, count)| **count >= 2).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 5);
    }
}
