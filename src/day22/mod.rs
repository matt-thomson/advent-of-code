mod step;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::StructOpt;

use crate::problem::Problem;

use step::Step;

#[derive(Debug, StructOpt)]
pub struct Day22 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(default_value = "10007")]
    num_cards: usize,
}

impl Problem for Day22 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let shuffled = self.run();
        let (position, _) = shuffled
            .iter()
            .enumerate()
            .find(|(_, x)| **x == 2019)
            .unwrap();

        position
    }

    fn part_two(&self) -> usize {
        unimplemented!();
    }
}

impl Day22 {
    fn run(&self) -> Vec<usize> {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        let mut deck: Vec<usize> = (0..self.num_cards).collect();

        for line in reader.lines() {
            let step = Step::parse(&line.unwrap());
            deck = step.apply(&deck);
        }

        deck
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = PathBuf::from("fixtures/day22a.txt");
        let problem = Day22 {
            input,
            num_cards: 10,
        };

        let result = problem.run();

        assert_eq!(result, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_run_b() {
        let input = PathBuf::from("fixtures/day22b.txt");
        let problem = Day22 {
            input,
            num_cards: 10,
        };

        let result = problem.run();

        assert_eq!(result, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_run_c() {
        let input = PathBuf::from("fixtures/day22c.txt");
        let problem = Day22 {
            input,
            num_cards: 10,
        };

        let result = problem.run();

        assert_eq!(result, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_run_d() {
        let input = PathBuf::from("fixtures/day22d.txt");
        let problem = Day22 {
            input,
            num_cards: 10,
        };

        let result = problem.run();

        assert_eq!(result, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
