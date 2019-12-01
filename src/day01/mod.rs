use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn part_one(input_path: &Path) -> u32 {
    let file = File::open(input_path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .map(fuel_required)
        .sum()
}

fn fuel_required(mass: u32) -> u32 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::{fuel_required, part_one};
    use std::path::Path;

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100_756), 33583);
    }

    #[test]
    fn test_part_one() {
        let input_path = Path::new("fixtures/day01/input.txt");
        assert_eq!(part_one(input_path), 34241);
    }
}
