use clap::Parser;

use aoc2021::{problems, Problem};

#[derive(Debug, Parser)]
enum Opts {
    Day01(crate::problems::day01::Day01),
}

fn main() {
    match Opts::parse() {
        Opts::Day01(day01) => day01.run(),
    }
}
