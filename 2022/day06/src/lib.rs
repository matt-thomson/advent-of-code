use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use eyre::{eyre, Result};

#[derive(Debug)]
pub struct Problem {
    input: String,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        Ok(Self { input })
    }

    pub fn part_one(&self) -> Result<usize> {
        let chars: Vec<char> = self.input.chars().collect();
        chars
            .windows(4)
            .enumerate()
            .find(|(_, window)| all_different(window))
            .map(|(index, _)| index + 4)
            .ok_or_else(|| eyre!("couldn't find start marker"))
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
    fn test_more_part_one_cases(#[case] input: String, #[case] expected: usize) {
        let problem = Problem { input };
        let result = problem.part_one().unwrap();

        assert_eq!(result, expected);
    }
}
