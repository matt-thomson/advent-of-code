mod dice;
mod state;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::dice::Dice;
use crate::state::State;

#[derive(Debug)]
pub struct Problem {
    starting_positions: [u32; 2],
}

const DIRAC: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];

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

    pub fn part_two(&self) -> u64 {
        let states = State::all(21);
        let target = State::new(&self.starting_positions);

        let mut winning_universes = HashMap::new();

        while !winning_universes.contains_key(&target) {
            for state in states.iter() {
                if winning_universes.contains_key(&state) {
                    continue;
                }

                let win_counts: Vec<_> = (3..=9)
                    .map(|roll| state.next(roll))
                    .map(|state| winning_universes_from(&state, &winning_universes, 21))
                    .collect();

                if win_counts.iter().all(|count| count.is_some()) {
                    let result = win_counts
                        .iter()
                        .zip(DIRAC)
                        .map(|(counts, multiplier)| counts.unwrap().map(|count| count * multiplier))
                        .fold([0, 0], |acc, counts| {
                            [acc[0] + counts[0], acc[1] + counts[1]]
                        });

                    winning_universes.insert(state, result);
                }
            }
        }

        *winning_universes[&target].iter().max().unwrap()
    }
}

fn winning_universes_from(
    state: &State,
    winning_universes: &HashMap<&State, [u64; 2]>,
    required: u32,
) -> Option<[u64; 2]> {
    if let Some(winner) = state.winner(required) {
        let mut result = [0, 0];
        result[winner] = 1;

        Some(result)
    } else {
        winning_universes.get(state).cloned()
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

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 444356092776315);
    }
}
