#[macro_use]
extern crate nom;

mod sue;

use sue::Sue;

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");

    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> u32 {
    let mut ticker = BTreeMap::new();

    ticker.insert("children".to_string(), equal(3));
    ticker.insert("cats".to_string(), equal(7));
    ticker.insert("samoyeds".to_string(), equal(2));
    ticker.insert("pomeranians".to_string(), equal(3));
    ticker.insert("akitas".to_string(), equal(0));
    ticker.insert("vizslas".to_string(), equal(0));
    ticker.insert("goldfish".to_string(), equal(5));
    ticker.insert("trees".to_string(), equal(3));
    ticker.insert("cars".to_string(), equal(2));
    ticker.insert("perfumes".to_string(), equal(1));

    solve(filename, ticker)
}

fn solve_part_two(filename: &str) -> u32 {
    let mut ticker = BTreeMap::new();

    ticker.insert("children".to_string(), equal(3));
    ticker.insert("cats".to_string(), greater_than(7));
    ticker.insert("samoyeds".to_string(), equal(2));
    ticker.insert("pomeranians".to_string(), less_than(3));
    ticker.insert("akitas".to_string(), equal(0));
    ticker.insert("vizslas".to_string(), equal(0));
    ticker.insert("goldfish".to_string(), less_than(5));
    ticker.insert("trees".to_string(), greater_than(3));
    ticker.insert("cars".to_string(), equal(2));
    ticker.insert("perfumes".to_string(), equal(1));

    solve(filename, ticker)
}

fn solve(filename: &str, ticker: BTreeMap<String, Box<Fn(u32) -> bool>>) -> u32 {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| Sue::parse(&line.unwrap()))
        .find(|sue| sue.matches(&ticker))
        .unwrap()
        .number()
}

fn equal(value: u32) -> Box<Fn(u32) -> bool> {
    Box::new(move |x| x == value)
}

fn greater_than(value: u32) -> Box<Fn(u32) -> bool> {
    Box::new(move |x| x > value)
}

fn less_than(value: u32) -> Box<Fn(u32) -> bool> {
    Box::new(move |x| x < value)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt"), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example.txt"), 4);
    }
}
