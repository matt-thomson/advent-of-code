mod maze;

use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

use maze::Maze;

#[derive(Debug, StructOpt)]
pub struct Day18 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day18 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let maze = Maze::read(&self.input);
        dbg!(maze);

        unimplemented!();
    }

    fn part_two(&self) -> usize {
        unimplemented!();
    }
}
