use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::StructOpt;

use crate::command;

#[derive(Debug, StructOpt)]
pub struct Day01 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl command::Command for Day01 {
    fn part_one(&self) -> u32 {
        self.solve(basic_fuel_required)
    }

    fn part_two(&self) -> u32 {
        self.solve(total_fuel_required)
    }
}

impl Day01 {
    fn solve(&self, fuel_required: fn(u32) -> u32) -> u32 {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|line| line.unwrap().parse::<u32>().unwrap())
            .map(fuel_required)
            .sum()
    }
}

fn basic_fuel_required(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn total_fuel_required(mass: u32) -> u32 {
    let mut total = 0;
    let mut last = mass;

    while last > 0 {
        last = basic_fuel_required(last);
        total += last;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::command::Command;

    #[test]
    fn test_basic_fuel_required() {
        assert_eq!(basic_fuel_required(2), 0);
        assert_eq!(basic_fuel_required(12), 2);
        assert_eq!(basic_fuel_required(14), 2);
        assert_eq!(basic_fuel_required(1969), 654);
        assert_eq!(basic_fuel_required(100_756), 33583);
    }

    #[test]
    fn test_total_fuel_required() {
        assert_eq!(total_fuel_required(14), 2);
        assert_eq!(total_fuel_required(1969), 966);
        assert_eq!(total_fuel_required(100_756), 50346);
    }

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day01.txt");
        let command = Day01 { input };

        assert_eq!(command.part_one(), 34241);
    }

    #[test]
    fn test_part_two() {
        let input = PathBuf::from("fixtures/day01.txt");
        let command = Day01 { input };

        assert_eq!(command.part_two(), 51316);
    }
}
