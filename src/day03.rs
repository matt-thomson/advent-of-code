use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::command;

#[derive(Debug, StructOpt)]
pub struct Day03 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl command::Command for Day03 {
    fn part_one(&self) -> u32 {
        let input = fs::read_to_string(&self.input).unwrap();
        let first = path(input.lines().nth(0).unwrap());
        let second = path(input.lines().nth(1).unwrap());

        first
            .intersection(&second)
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap() as u32
    }

    fn part_two(&self) -> u32 {
        unimplemented!()
    }
}

fn path(instructions: &str) -> HashSet<(i32, i32)> {
    let mut position = (0, 0);
    let mut result = HashSet::new();

    for instruction in instructions.split(',') {
        let visited = step(position, instruction);
        position = visited[visited.len() - 1];

        for point in visited {
            result.insert(point);
        }
    }

    result
}

fn step(start: (i32, i32), instruction: &str) -> Vec<(i32, i32)> {
    let (x, y) = start;
    let (direction, length_str) = instruction.split_at(1);
    let length: i32 = length_str.parse().unwrap();

    let mut result = vec![];

    for step in 1..=length {
        let next = match direction {
            "D" => (x, y - step),
            "L" => (x - step, y),
            "R" => (x + step, y),
            "U" => (x, y + step),
            _ => panic!("unknown direction {}", direction),
        };

        result.push(next);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::command::Command;

    #[test]
    fn test_step_right() {
        let start = (0, 0);
        let instruction = "R8";

        let result = step(start, instruction);
        let expected = vec![
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_step_up() {
        let start = (8, 0);
        let instruction = "U5";

        let result = step(start, instruction);
        let expected = vec![(8, 1), (8, 2), (8, 3), (8, 4), (8, 5)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_step_left() {
        let start = (8, 5);
        let instruction = "L5";

        let result = step(start, instruction);
        let expected = vec![(7, 5), (6, 5), (5, 5), (4, 5), (3, 5)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_step_down() {
        let start = (3, 5);
        let instruction = "D6";

        let result = step(start, instruction);
        let expected = vec![(3, 4), (3, 3), (3, 2), (3, 1), (3, 0), (3, -1)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_path() {
        let instructions = "R8,U5,L5,D6";

        let result = path(instructions);

        assert!(result.contains(&(3, -1)));
    }

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day03a.txt");
        let command = Day03 { input };

        assert_eq!(command.part_one(), 6);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day03b.txt");
        let command = Day03 { input };

        assert_eq!(command.part_one(), 159);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day03c.txt");
        let command = Day03 { input };

        assert_eq!(command.part_one(), 135);
    }
}
