mod position;
mod target;
mod trajectory;

use std::fs;
use std::path::Path;

use target::Target;
use trajectory::{Outcome, Trajectory};

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
        let mut trajectory = Trajectory::new(dx, dy);

        loop {
            if let Some(outcome) = trajectory.outcome(&self.target) {
                return outcome;
            }

            trajectory = trajectory.step();
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
