use std::collections::BTreeMap;
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
    fn part_one(&self) -> u32 {
        let orbits = read_orbits(&self.input);
        orbits
            .keys()
            .map(|object| orbit_length(&orbits, object))
            .sum()
    }

    fn part_two(&self) -> u32 {
        unimplemented!();
    }
}

fn read_orbits(path: &Path) -> BTreeMap<String, String> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let mut result = BTreeMap::new();

    for line_opt in reader.lines() {
        let line = line_opt.unwrap();
        let index = line.find(')').unwrap();
        let inner = line[0..index].to_string();
        let outer = line[(index + 1)..].to_string();

        result.insert(outer, inner);
    }

    result
}

fn orbit_length(orbits: &BTreeMap<String, String>, object: &str) -> u32 {
    let mut result = 0;
    let mut current = object;

    while let Some(inner) = orbits.get(current) {
        current = inner;
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::command::Command;

    #[test]
    fn test_read_orbits() {
        let path = PathBuf::from("fixtures/day06.txt");
        let orbits = read_orbits(&path);

        assert_eq!(orbits.get("B").unwrap(), "COM");
        assert_eq!(orbits.get("J").unwrap(), "E");
    }

    #[test]
    fn test_orbit_length() {
        let path = PathBuf::from("fixtures/day06.txt");
        let orbits = read_orbits(&path);

        assert_eq!(orbit_length(&orbits, "D"), 3);
        assert_eq!(orbit_length(&orbits, "L"), 7);
        assert_eq!(orbit_length(&orbits, "COM"), 0);
    }

    #[test]
    fn test_part_one() {
        let input = PathBuf::from("fixtures/day06.txt");
        let command = Day06 { input };

        assert_eq!(command.part_one(), 42);
    }
}
