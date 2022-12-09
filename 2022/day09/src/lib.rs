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
        let directions = input
            .lines()
            .flat_map(|line| parse_line(line).unwrap())
            .collect();

        Ok(Self { directions })
    }

    pub fn part_one(&self) -> usize {
        let mut rope = Rope::<1>::new();
        let mut visited: HashSet<_> = rope.tails().into_iter().collect();

        for direction in self.directions.iter() {
            rope.step(&direction);
            visited.extend(rope.tails());
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
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one();

        assert_eq!(result, 13);
    }
}
