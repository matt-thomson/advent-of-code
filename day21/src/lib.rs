mod dice;
mod state;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::dice::Dice;
use crate::state::State;

#[derive(Debug)]
pub struct Problem {
    starting_positions: [u32; 2],
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let starting_positions = reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self { starting_positions }
    }

    pub fn part_one(&self) -> u32 {
        let mut dice = Dice::new();
        let mut state = State::new(&self.starting_positions);

        while state.winner(1000).is_none() {
            state = state.next(dice.roll());
        }

        let score = state.score(1 - state.winner(1000).unwrap());
        score * dice.rolls
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 739785);
    }
}
