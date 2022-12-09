mod direction;
mod rope;

use std::collections::HashSet;
use std::path::Path;
use std::{fs, iter::repeat};

use eyre::{eyre, Result};

use crate::direction::Direction;
use crate::rope::Rope;

#[derive(Debug)]
pub struct Problem {
    directions: Vec<Direction>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let directions = input.lines().flat_map(parse_line).flatten().collect();

        Ok(Self { directions })
    }

    pub fn part_one(&self) -> usize {
        self.solve::<2>()
    }

    pub fn part_two(&self) -> usize {
        self.solve::<10>()
    }

    fn solve<const N: usize>(&self) -> usize {
        let mut rope = Rope::<N>::default();

        let mut visited: HashSet<_> = HashSet::new();
        visited.insert(rope.tail());

        for direction in &self.directions {
            rope.step(direction);
            visited.insert(rope.tail());
        }

        visited.len()
    }
}

fn parse_line(line: &str) -> Result<impl Iterator<Item = Direction>> {
    let (direction, count) = line
        .split_once(' ')
        .ok_or_else(|| eyre!("can't parse line without space"))?;

    let direction: Direction = direction.parse()?;
    let count: usize = count.parse()?;

    Ok(repeat(direction).take(count))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example1.txt").unwrap();
        let result = problem.part_one();

        assert_eq!(result, 13);
    }

    #[rstest]
    #[case("example1.txt", 1)]
    #[case("example2.txt", 36)]
    fn test_part_two(#[case] filename: String, #[case] expected: usize) {
        let problem = Problem::new(filename).unwrap();
        let result = problem.part_two();

        assert_eq!(result, expected);
    }
}
