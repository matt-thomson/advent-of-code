mod board;
mod game;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use board::Board;
use game::Game;

#[derive(Debug)]
pub struct Problem {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let lines: Vec<_> = BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        let numbers = lines[0]
            .split(',')
            .map(|number| number.parse().unwrap())
            .collect();

        let boards = lines[1..].chunks(5).map(Board::new).collect();

        Self { numbers, boards }
    }

    pub fn part_one(&self) -> u32 {
        let mut games: Vec<_> = self.boards.iter().map(Game::new).collect();

        for number in &self.numbers {
            for game in &mut games {
                game.mark(*number);

                if game.is_winner() {
                    return game.score() * number;
                }
            }
        }

        panic!("Couldn't find winning board")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 4512);
    }
}
