mod monkey;
mod operation;

use std::fs;
use std::path::Path;

use eyre::Result;

use crate::monkey::Monkey;

#[derive(Debug)]
pub struct Problem {
    monkeys: Vec<Monkey>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let monkeys = Monkey::parse_all(&input)?;

        Ok(Self { monkeys })
    }

    pub fn part_one(&self) -> u64 {
        self.solve(20, 3)
    }

    pub fn part_two(&self) -> u64 {
        self.solve(10000, 1)
    }

    fn solve(&self, rounds: usize, divisor: u64) -> u64 {
        let modulus = self.monkeys.iter().map(Monkey::divisible_test).product();

        let mut items: Vec<_> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.starting_items().to_vec())
            .collect();

        let mut activity = vec![0_u64; self.monkeys.len()];

        for _ in 0..rounds {
            for i in 0..items.len() {
                let results: Vec<_> = items[i]
                    .iter()
                    .map(|worry_level| self.monkeys[i].inspect(*worry_level, divisor, modulus))
                    .collect();

                for (worry_level, destination) in results {
                    items[destination].push(worry_level);
                    activity[i] += 1;
                }

                items[i] = vec![];
            }
        }

        activity.sort_unstable();
        activity.reverse();

        activity[0] * activity[1]
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one();

        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_two();

        assert_eq!(result, 2713310158);
    }
}
