mod dice;
mod player;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::dice::Dice;
use crate::player::Player;

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
        let mut players: [Player; 2] = self.starting_positions.map(Player::new);

        for i in [0, 1].into_iter().cycle() {
            players[i].step(dice.roll());

            if players[i].score >= 1000 {
                return players[1 - i].score * dice.rolls;
            }
        }

        unreachable!()
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
