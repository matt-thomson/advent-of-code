mod digit;
mod display;

use std::fs;
use std::path::Path;

use display::Display;

#[derive(Debug)]
pub struct Problem {
    displays: Vec<Display>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let displays = fs::read_to_string(&path)
            .unwrap()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        Self { displays }
    }

    pub fn part_one(&self) -> usize {
        self.displays
            .iter()
            .flat_map(|display| display.output())
            .filter(|digit| [1, 4, 7, 8].contains(digit))
            .count()
    }

    pub fn part_two(&self) -> usize {
        self.displays.iter().map(|display| display.total()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 26);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 61229);
    }
}
