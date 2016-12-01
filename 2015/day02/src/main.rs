mod present;

use present::Present;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).unwrap();
    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(path: &str) -> u32 {
    solve(path, &|present| present.wrapping_paper())
}

fn solve_part_two(path: &str) -> u32 {
    solve(path, &|present| present.ribbon())
}

fn solve(path: &str, op: &Fn(Present) -> u32) -> u32 {
    BufReader::new(File::open(path).unwrap()).lines()
        .map(|line| op(Present::new(&line.unwrap())))
        .fold(0, |acc, i| acc + i)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt"), 101);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example.txt"), 48);
    }
}
