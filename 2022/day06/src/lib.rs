use std::convert::Infallible;
use std::fs;
use std::path::Path;
use std::{collections::BTreeSet, str::FromStr};

use eyre::{eyre, Result};

#[derive(Debug)]
pub struct Problem {
    characters: Vec<char>,
}

impl FromStr for Problem {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            characters: input.chars().collect(),
        })
    }
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        Ok(input.parse()?)
    }

    pub fn part_one(&self) -> Result<usize> {
        self.solve(4)
    }

    pub fn part_two(&self) -> Result<usize> {
        self.solve(14)
    }

    fn solve(&self, marker_size: usize) -> Result<usize> {
        self.characters
            .windows(marker_size)
            .enumerate()
            .find(|(_, window)| all_different(window))
            .map(|(index, _)| index + marker_size)
            .ok_or_else(|| eyre!("couldn't find marker"))
    }
}

fn all_different(items: &[char]) -> bool {
    let set = BTreeSet::from_iter(items.iter());
    set.len() == items.len()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one().unwrap();

        assert_eq!(result, 7);
    }

    #[rstest]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_more_part_one_cases(#[case] problem: Problem, #[case] expected: usize) {
        let result = problem.part_one().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_two().unwrap();

        assert_eq!(result, 19);
    }

    #[rstest]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_more_part_two_cases(#[case] problem: Problem, #[case] expected: usize) {
        let result = problem.part_two().unwrap();
        assert_eq!(result, expected);
    }
}
