use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn part_one(input_path: &Path) -> u32 {
    solve(input_path, basic_fuel_required)
}

pub fn part_two(input_path: &Path) -> u32 {
    solve(input_path, total_fuel_required)
}

fn solve(input_path: &Path, fuel_required: fn(u32) -> u32) -> u32 {
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .map(fuel_required)
        .sum()
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
    use std::path::Path;

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
        let input_path = Path::new("fixtures/day01/input.txt");
        assert_eq!(part_one(input_path), 34241);
    }

    #[test]
    fn test_part_two() {
        let input_path = Path::new("fixtures/day01/input.txt");
        assert_eq!(part_two(input_path), 51316);
    }
}
