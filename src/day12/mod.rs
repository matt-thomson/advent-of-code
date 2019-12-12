mod planet;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

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
        let mut planets = read(&self.input);
        let original: Vec<_> = planets.iter().cloned().collect();

        for i in 1.. {
            step(&mut planets);

            if original == planets {
                return i;
            }
        }

        unreachable!();
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
    let copy: Vec<_> = planets.iter().cloned().collect();

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
}
