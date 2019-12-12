mod planet;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

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
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        let mut planets: Vec<_> = reader
            .lines()
            .map(|line| Planet::parse(&line.unwrap()))
            .collect();

        for _ in 0..self.steps {
            for (i, first) in planets.clone().iter().enumerate() {
                for (j, second) in planets.iter_mut().enumerate() {
                    if i != j {
                        second.apply_gravity(&first);
                    }
                }
            }

            for planet in &mut planets {
                planet.step();
            }
        }

        planets.iter().map(|planet| planet.energy()).sum()
    }

    fn part_two(&self) -> usize {
        unimplemented!();
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
}
