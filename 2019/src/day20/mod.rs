mod maze;

use std::path::PathBuf;

use pathfinding::prelude::bfs;
use structopt::StructOpt;

use crate::problem::Problem;

use maze::Maze;

#[derive(Debug, StructOpt)]
pub struct Day20 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day20 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let maze = Maze::read(&self.input);

        let result = bfs(
            maze.start(),
            |position| maze.neighbours(position),
            |position| position == maze.end(),
        );

        result.unwrap().len() - 1
    }

    fn part_two(&self) -> usize {
        let maze = Maze::read(&self.input);

        let result = bfs(
            &(*maze.start(), 0),
            |position| maze.neighbours_with_level(position),
            |position| position == &(*maze.end(), 0),
        );

        result.unwrap().len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day20a.txt");
        let problem = Day20 { input };

        assert_eq!(problem.part_one(), 23);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day20b.txt");
        let problem = Day20 { input };

        assert_eq!(problem.part_one(), 58);
    }

    #[test]
    fn test_part_two_c() {
        let input = PathBuf::from("fixtures/day20c.txt");
        let problem = Day20 { input };

        assert_eq!(problem.part_two(), 396);
    }
}
