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

    pub fn part_one(&self) -> i32 {
        (1..=500)
            .flat_map(|dx| (1..=500).map(move |dy| (dx, dy)))
            .filter_map(|(dx, dy)| self.trajectory(dx, dy).max_height())
            .max()
            .unwrap()
    }

    pub fn part_two(&self) -> usize {
        (1..=500)
            .flat_map(|dx| (-500..=500).map(move |dy| (dx, dy)))
            .filter_map(|(dx, dy)| self.trajectory(dx, dy).max_height())
            .count()
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

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 112);
    }
}
