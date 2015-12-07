#[macro_use]
extern crate nom;

mod circuit;
mod gate;

use circuit::Circuit;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> u16 {
    let circuit = read_circuit(&filename);
    circuit.result("a")
}

fn solve_part_two(filename: &str) -> u16 {
    let circuit = read_circuit(&filename);
    let part1_result = circuit.result("a");
    circuit.result_with_override(part1_result, "a")
}

fn read_circuit(filename: &str) -> Circuit {
    let file = BufReader::new(File::open(filename).unwrap());
    Circuit::parse(&mut file.lines().map(|line| line.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one("data/example.txt");
        assert_eq!(result, 507);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two("data/example.txt");
        assert_eq!(result, 507);
    }
}
