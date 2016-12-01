#[macro_use]
extern crate nom;

mod reindeer;

use reindeer::Reindeer;

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    let time = env::args().nth(2).expect("Must supply a time").parse().unwrap();

    println!("{}", solve_part_one(&filename, time));
    println!("{}", solve_part_two(&filename, time));
}

fn solve_part_one(filename: &str, time: u32) -> u32 {
    let file = BufReader::new(File::open(filename).unwrap());

    file.lines().map(|line| Reindeer::parse(&line.unwrap()).distance(time)).max().unwrap()
}

fn solve_part_two(filename: &str, time: u32) -> u32 {
    let file = BufReader::new(File::open(filename).unwrap());
    let reindeer = file.lines().map(|line| Reindeer::parse(&line.unwrap())).collect::<Vec<_>>();

    let mut scores = BTreeMap::new();

    for i in 1..(time + 1) {
        let mut distances = BTreeMap::new();
        for r in reindeer.iter() {
            distances.insert(r.name(), r.distance(i));
        }

        let furthest = *distances.values().max().unwrap();

        for (name, distance) in distances {
            if distance == furthest {
                *scores.entry(name).or_insert(0) += 1;
            }
        }
    }

    *scores.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt", 1000), 1120);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example.txt", 1000), 689);
    }
}
