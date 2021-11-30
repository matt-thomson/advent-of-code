mod problems;

use clap::Parser;

use std::fmt::Display;

#[derive(Debug, Parser)]
enum Opts {
    Day01(problems::day01::Day01),
}

pub trait Problem {
    type Output: Display;

    fn part_one(&self) -> Self::Output;
    fn part_two(&self) -> Self::Output;

    fn run(&self) {
        println!("{}", self.part_one());
        println!("{}", self.part_two());
    }
}

fn main() {
    match Opts::parse() {
        Opts::Day01(day01) => day01.run(),
    }
}
