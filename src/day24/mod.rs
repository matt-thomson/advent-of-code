mod board;

use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day24 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day24 {
    type Output = u32;

    fn part_one(&self) -> u32 {
        unimplemented!();
    }

    fn part_two(&self) -> u32 {
        unimplemented!();
    }
}
