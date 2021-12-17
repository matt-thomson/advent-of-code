mod position;
mod target;

use std::fs;
use std::path::Path;

use target::{Outcome, Target};

use crate::position::Position;

#[derive(Debug)]
pub struct Problem {
    target: Target,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let input = fs::read_to_string(path).unwrap();
        let target = input.trim().parse().unwrap();

        Self { target }
    }

    pub fn part_one(&self) -> usize {
        dbg!(self.trajectory(7, 2));

        unimplemented!()
    }

    fn trajectory(&self, dx: i32, dy: i32) -> Outcome {
        let mut position = Position::new(dx, dy);

        loop {
            if let Some(outcome) = self.target.outcome(&position) {
                return outcome;
            }

            position = position.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 45);
    }
}
