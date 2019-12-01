use structopt::StructOpt;

use crate::day01;

#[derive(Debug, StructOpt)]
enum Opts {
    Day01(day01::Day01),
}

pub trait Command {
    fn part_one(&self) -> u32;
    fn part_two(&self) -> u32;
}

impl dyn Command {
    pub fn from_args() -> Box<dyn Command> {
        match Opts::from_args() {
            Opts::Day01(command) => Box::new(command),
        }
    }
}
