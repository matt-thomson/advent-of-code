mod planet;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use num::Integer;
use structopt::StructOpt;

use crate::problem::Problem;

use planet::Planet;

#[derive(Debug, StructOpt)]
pub struct Day12 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(default_value = "1000")]
    steps: usize,
}

impl Problem for Day12 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let mut planets = read(&self.input);

        for _ in 0..self.steps {
            step(&mut planets);
        }

        planets.iter().map(|planet| planet.energy()).sum()
    }

    fn part_two(&self) -> usize {
        let planets = read(&self.input);
        let mut result: usize = 1;

        for i in 0..3 {
            result = result.lcm(&find_cycle(&planets, i));
        }

        result
    }
}

fn read(input: &Path) -> Vec<Planet> {
    let file = File::open(&input).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| Planet::parse(&line.unwrap()))
        .collect()
}

fn step(planets: &mut [Planet]) {
    let copy = planets.to_vec();

    for (i, first) in copy.iter().enumerate() {
        for (j, second) in planets.iter_mut().enumerate() {
            if i != j {
                second.apply_gravity(&first);
            }
        }
    }

    for planet in &mut planets.iter_mut() {
        planet.step();
    }
}

fn find_cycle(planets: &[Planet], dimension: usize) -> usize {
    let mut current = planets.to_vec();

    for i in 1.. {
        step(&mut current);

        if dimension_matches(&planets, &current, dimension) {
            return i;
        }
    }

    unreachable!();
}

fn dimension_matches(first: &[Planet], second: &[Planet], dimension: usize) -> bool {
    first.iter().zip(second).all(|(first, second)| {
        first.position(dimension) == second.position(dimension)
            && first.velocity(dimension) == second.velocity(dimension)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day12a.txt");
        let problem = Day12 { input, steps: 10 };

        assert_eq!(problem.part_one(), 179);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day12b.txt");
        let problem = Day12 { input, steps: 100 };

        assert_eq!(problem.part_one(), 1940);
    }

    #[test]
    fn test_part_two_a() {
        let input = PathBuf::from("fixtures/day12a.txt");
        let problem = Day12 { input, steps: 10 };

        assert_eq!(problem.part_two(), 2772);
    }

    #[test]
    fn test_part_two_b() {
        let input = PathBuf::from("fixtures/day12b.txt");
        let problem = Day12 { input, steps: 10 };

        assert_eq!(problem.part_two(), 4_686_774_924);
    }
}
