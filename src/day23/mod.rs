use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day23 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day23 {
    type Output = i64;

    fn part_one(&self) -> i64 {
        unimplemented!();
    }

    fn part_two(&self) -> i64 {
        unimplemented!();
    }
}
