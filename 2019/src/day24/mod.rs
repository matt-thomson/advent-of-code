mod board;
mod multiboard;

use std::collections::HashSet;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

use board::Board;
use multiboard::MultiBoard;

#[derive(Debug, StructOpt)]
pub struct Day24 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(default_value = "200")]
    time: usize,
}

impl Problem for Day24 {
    type Output = u32;

    fn part_one(&self) -> u32 {
        let mut board = Board::read(&self.input);
        let mut seen = HashSet::new();

        loop {
            let biodiversity = board.biodiversity();

            if seen.contains(&biodiversity) {
                return biodiversity;
            }

            seen.insert(biodiversity);
            board = board.step();
        }
    }

    fn part_two(&self) -> u32 {
        let mut board = MultiBoard::read(&self.input);

        for _ in 0..self.time {
            board = board.step();
        }

        board.num_bugs() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day24.txt");
        let problem = Day24 { input, time: 10 };

        assert_eq!(problem.part_one(), 2_129_920);
    }

    #[test]
    fn test_part_two() {
        let input = PathBuf::from("fixtures/day24.txt");
        let problem = Day24 { input, time: 10 };

        assert_eq!(problem.part_two(), 99);
    }
}
