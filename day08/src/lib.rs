use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Problem {
    displays: Vec<Vec<String>>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let displays = fs::read_to_string(&path)
            .unwrap()
            .lines()
            .map(parse_display)
            .collect();

        Self { displays }
    }

    pub fn part_one(&self) -> usize {
        self.displays
            .iter()
            .flatten()
            .map(|digit| digit.len())
            .filter(|length| [2, 3, 4, 7].contains(length))
            .count()
    }
}

fn parse_display(line: &str) -> Vec<String> {
    let (_, output) = line.split_once('|').unwrap();
    output
        .trim()
        .split_whitespace()
        .map(|digit| digit.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 26);
    }
}
