use structopt::StructOpt;

use crate::day01;
use crate::day02;
use crate::day03;

#[derive(Debug, StructOpt)]
enum Opts {
    Day01(day01::Day01),
    Day02(day02::Day02),
    Day03(day03::Day03),
}

pub trait Command {
    fn part_one(&self) -> u32;
    fn part_two(&self) -> u32;
}

impl dyn Command {
    pub fn from_args() -> Box<dyn Command> {
        match Opts::from_args() {
            Opts::Day01(command) => Box::new(command),
            Opts::Day02(command) => Box::new(command),
            Opts::Day03(command) => Box::new(command),
        }
    }
}
