use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use crate::command;

#[derive(Debug, StructOpt)]
pub struct Day06 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl command::Command for Day06 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let orbits = read_orbits(&self.input);
        orbits
            .keys()
            .map(|object| indirect_orbits(&orbits, object).len())
            .sum()
    }

    fn part_two(&self) -> usize {
        let orbits = read_orbits(&self.input);
        let first = indirect_orbits(&orbits, "YOU");
        let second = indirect_orbits(&orbits, "SAN");

        for offset in 0.. {
            if first[first.len() - offset - 1] != second[second.len() - offset - 1] {
                return first.len() + second.len() - (2 * offset);
            }
        }

        unreachable!();
    }
}

fn read_orbits(path: &Path) -> HashMap<String, String> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let mut result = HashMap::new();

    for line_opt in reader.lines() {
        let line = line_opt.unwrap();
        let index = line.find(')').unwrap();
        let inner = line[0..index].to_string();
        let outer = line[(index + 1)..].to_string();

        result.insert(outer, inner);
    }

    result
}

fn indirect_orbits<'a>(orbits: &'a HashMap<String, String>, object: &str) -> Vec<&'a String> {
    let mut result = vec![];
    let mut current = object;

    while let Some(inner) = orbits.get(current) {
        current = inner;
        result.push(inner);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::command::Command;

    #[test]
    fn test_read_orbits() {
        let path = PathBuf::from("fixtures/day06a.txt");
        let orbits = read_orbits(&path);

        assert_eq!(orbits.get("B").unwrap(), "COM");
        assert_eq!(orbits.get("J").unwrap(), "E");
    }

    #[test]
    fn test_indirect_orbits() {
        let path = PathBuf::from("fixtures/day06a.txt");
        let orbits = read_orbits(&path);

        assert_eq!(indirect_orbits(&orbits, "D"), vec!["C", "B", "COM"]);
        assert_eq!(
            indirect_orbits(&orbits, "L"),
            vec!["K", "J", "E", "D", "C", "B", "COM"]
        );
        assert!(indirect_orbits(&orbits, "COM").is_empty());
    }

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day06a.txt");
        let command = Day06 { input };

        assert_eq!(command.part_one(), 42);
    }

    #[test]
    fn test_part_two() {
        let input = PathBuf::from("fixtures/day06b.txt");
        let command = Day06 { input };

        assert_eq!(command.part_two(), 4);
    }
}
