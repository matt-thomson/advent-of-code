use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day20 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day20 {
    type Output = usize;

    fn part_one(&self) -> usize {
        unimplemented!();
    }

    fn part_two(&self) -> usize {
        unimplemented!();
    }
}
