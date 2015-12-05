#![feature(io)]

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let filename = env::args().nth(1).unwrap();
    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> usize {
    let directions = File::open(filename).unwrap().chars().map(|c| c.unwrap());
    let mut position = (0, 0);

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for direction in directions.into_iter() {
        position = next_position(&position, &direction);
        visited.insert(position);
    }

    visited.len()
}

fn solve_part_two(filename: &str) -> usize {
    let directions = File::open(filename).unwrap().chars().map(|c| c.unwrap());
    let mut santa_position = (0, 0);
    let mut robosanta_position = (0, 0);

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for (index, direction) in directions.into_iter().enumerate() {
        println!("{:?}", direction);
        if index % 2 == 0 {
            santa_position = next_position(&santa_position, &direction);
            visited.insert(santa_position);
            println!("{:?}", santa_position);
        } else {
            robosanta_position = next_position(&robosanta_position, &direction);
            visited.insert(robosanta_position);
            println!("{:?}", robosanta_position);
        }
    }

    visited.len()
}

fn next_position(current: &(i32, i32), direction: &char) -> (i32, i32) {
    let (x, y) = *current;

    match *direction {
        '^' => (x, y + 1),
        '>' => (x + 1, y),
        'v' => (x, y - 1),
        '<' => (x - 1, y),
        _   => (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one_example_1() {
        assert_eq!(solve_part_one("data/example1.txt"), 2);
    }

    #[test]
    fn test_part_one_example_2() {
        assert_eq!(solve_part_one("data/example2.txt"), 4);
    }

    #[test]
    fn test_part_one_example_3() {
        assert_eq!(solve_part_one("data/example3.txt"), 2);
    }

    #[test]
    fn test_part_two_example_2() {
        assert_eq!(solve_part_two("data/example2.txt"), 3);
    }

    #[test]
    fn test_part_two_example_3() {
        assert_eq!(solve_part_two("data/example3.txt"), 11);
    }

    #[test]
    fn test_part_two_example_4() {
        assert_eq!(solve_part_two("data/example4.txt"), 3);
    }
}
