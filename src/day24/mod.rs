mod board;

use std::collections::HashSet;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

use board::Board;

#[derive(Debug, StructOpt)]
pub struct Day24 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
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
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day24.txt");
        let problem = Day24 { input };

        assert_eq!(problem.part_one(), 2_129_920);
    }
}
