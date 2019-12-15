mod chemical;
mod reaction;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use crate::problem::Problem;

use reaction::Reaction;

#[derive(Debug, StructOpt)]
pub struct Day14 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(default_value = "1000000000000")]
    ore: u64,
}

impl Problem for Day14 {
    type Output = u64;

    fn part_one(&self) -> u64 {
        let reactions = read(&self.input);
        let mut products = HashMap::new();

        ore_required(&reactions, 1, "FUEL", &mut products)
    }

    fn part_two(&self) -> u64 {
        let reactions = read(&self.input);

        let mut low = 0;
        let mut high = self.ore;

        loop {
            let test = (low + high) / 2;
            let mut products = HashMap::new();

            if high == low + 1 {
                return low;
            }

            if ore_required(&reactions, test, "FUEL", &mut products) > self.ore {
                high = test;
            } else {
                low = test;
            }
        }
    }
}

fn read(path: &Path) -> HashMap<String, Reaction> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| Reaction::parse(&line.unwrap()))
        .map(|reaction| (reaction.output().name().to_string(), reaction))
        .collect()
}

fn ore_required(
    reactions: &HashMap<String, Reaction>,
    amount: u64,
    name: &str,
    products: &mut HashMap<String, u64>,
) -> u64 {
    let mut amount = amount;

    if name == "ORE" {
        return amount;
    }

    let product = products.entry(name.to_string()).or_insert(0);

    if *product >= amount {
        *product -= amount;
        return 0;
    } else {
        amount -= *product;
        *product = 0;
    }

    let reaction = reactions.get(name).unwrap();
    let num_reactions = ((amount - 1) / reaction.output().amount()) + 1;

    let surplus = (num_reactions * reaction.output().amount()) - amount;
    *product += surplus;

    let mut ore = 0;

    for input in reaction.inputs() {
        ore += ore_required(
            &reactions,
            input.amount() * num_reactions,
            input.name(),
            products,
        );
    }

    ore
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let path = PathBuf::from("fixtures/day14a.txt");
        let reactions = read(&path);

        let reaction = reactions.get("FUEL").unwrap();

        assert_eq!(reaction.inputs()[0].amount(), 7);
        assert_eq!(reaction.inputs()[0].name(), "A");

        assert_eq!(reaction.inputs()[1].amount(), 1);
        assert_eq!(reaction.inputs()[1].name(), "E");

        assert_eq!(reaction.output().amount(), 1);
        assert_eq!(reaction.output().name(), "FUEL");
    }

    #[test]
    fn test_part_one_a() {
        let input = PathBuf::from("fixtures/day14a.txt");
        let problem = Day14 {
            input,
            ore: 1_000_000_000_000,
        };

        assert_eq!(problem.part_one(), 31);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day14b.txt");
        let problem = Day14 {
            input,
            ore: 1_000_000_000_000,
        };

        assert_eq!(problem.part_one(), 165);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day14c.txt");
        let problem = Day14 {
            input,
            ore: 1_000_000_000_000,
        };

        assert_eq!(problem.part_one(), 13312);
    }

    #[test]
    fn test_part_one_d() {
        let input = PathBuf::from("fixtures/day14d.txt");
        let problem = Day14 {
            input,
            ore: 1_000_000_000_000,
        };

        assert_eq!(problem.part_one(), 180_697);
    }

    #[test]
    fn test_part_one_e() {
        let input = PathBuf::from("fixtures/day14e.txt");
        let problem = Day14 {
            input,
            ore: 1_000_000_000_000,
        };

        assert_eq!(problem.part_one(), 2_210_736);
    }

    #[test]
    fn test_part_two_c() {
        let input = PathBuf::from("fixtures/day14c.txt");
        let problem = Day14 {
            input,
            ore: 1_000_000_000_000,
        };

        assert_eq!(problem.part_two(), 82_892_753);
    }

    #[test]
    fn test_part_two_d() {
        let input = PathBuf::from("fixtures/day14d.txt");
        let problem = Day14 {
            input,
            ore: 1_000_000_000_000,
        };

        assert_eq!(problem.part_two(), 5_586_022);
    }

    #[test]
    fn test_part_two_e() {
        let input = PathBuf::from("fixtures/day14e.txt");
        let problem = Day14 {
            input,
            ore: 1_000_000_000_000,
        };

        assert_eq!(problem.part_two(), 460_664);
    }
}
