use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day03 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day03 {
    type Output = i32;

    fn part_one(&self) -> i32 {
        self.intersections()
            .keys()
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap()
    }

    fn part_two(&self) -> i32 {
        *self.intersections().values().min().unwrap() as i32
    }
}

impl Day03 {
    fn intersections(&self) -> HashMap<(i32, i32), usize> {
        let input = fs::read_to_string(&self.input).unwrap();
        let mut input_lines = input.lines();

        let first_path = path(input_lines.next().unwrap());
        let second_path = path(input_lines.next().unwrap());

        let mut result = HashMap::new();

        for (position, first_time) in first_path.into_iter() {
            if let Some(second_time) = second_path.get(&position) {
                result.insert(position, first_time + second_time);
            }
        }

        result
    }
}

fn path(instructions: &str) -> HashMap<(i32, i32), usize> {
    let mut position = (0, 0);
    let mut result = HashMap::new();
    let mut time = 1;

    for instruction in instructions.split(',') {
        let visited = step(position, instruction);
        position = visited[visited.len() - 1];

        for point in visited {
            result.entry(point).or_insert(time);
            time += 1;
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

    use crate::problem::Problem;

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

        assert_eq!(*result.get(&(3, -1)).unwrap(), 24);
    }

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day03a.txt");
        let problem = Day03 { input };

        assert_eq!(problem.part_one(), 6);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day03b.txt");
        let problem = Day03 { input };

        assert_eq!(problem.part_one(), 159);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day03c.txt");
        let problem = Day03 { input };

        assert_eq!(problem.part_one(), 135);
    }

    #[test]
    fn test_part_two_a() {
        let input = PathBuf::from("fixtures/day03a.txt");
        let problem = Day03 { input };

        assert_eq!(problem.part_two(), 30);
    }

    #[test]
    fn test_part_two_b() {
        let input = PathBuf::from("fixtures/day03b.txt");
        let problem = Day03 { input };

        assert_eq!(problem.part_two(), 610);
    }

    #[test]
    fn test_part_two_c() {
        let input = PathBuf::from("fixtures/day03c.txt");
        let problem = Day03 { input };

        assert_eq!(problem.part_two(), 410);
    }
}
