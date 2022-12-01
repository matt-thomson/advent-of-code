mod fold;
mod paper;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use fold::Fold;
use paper::Paper;

#[derive(Debug)]
pub struct Problem {
    paper: Paper,
    folds: Vec<Fold>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());

        let paper = Paper::parse(&mut lines.by_ref().take_while(|line| !line.is_empty()));
        let folds = lines.map(|line| line.parse().unwrap()).collect();

        Self { paper, folds }
    }

    pub fn part_one(&self) -> usize {
        self.paper.apply_fold(&self.folds[0]).num_dots()
    }

    pub fn part_two(&self) -> Paper {
        self.folds
            .iter()
            .fold(self.paper.clone(), |paper, fold| paper.apply_fold(fold))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 17);
    }
}
