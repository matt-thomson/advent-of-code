#![feature(iter_arith)]

#[macro_use]
extern crate nom;

mod instruction;

use instruction::Instruction;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> usize {
    let mut grid = [[false; 1000]; 1000];

    let file = BufReader::new(File::open(filename).unwrap());
    for instruction in file.lines().map(|line| Instruction::parse(&line.unwrap())) {
        instruction.english(&mut grid);
    }

    grid.iter().map(|column| column.iter().filter(|x| **x).count()).sum()
}

fn solve_part_two(filename: &str) -> u32 {
    let mut grid = [[0; 1000]; 1000];

    let file = BufReader::new(File::open(filename).unwrap());
    for instruction in file.lines().map(|line| Instruction::parse(&line.unwrap())) {
        instruction.elvish(&mut grid);
    }

    grid.iter().map(|column| column.iter().map(|x| *x as u32).sum::<u32>()).sum()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt"), 998996);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example.txt"), 1001996);
    }
}
