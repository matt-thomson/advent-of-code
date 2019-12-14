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
}

impl Problem for Day14 {
    type Output = u32;

    fn part_one(&self) -> u32 {
        let reactions = read(&self.input);
        let mut products = HashMap::new();

        ore_required(&reactions, 1, "FUEL", &mut products)
    }

    fn part_two(&self) -> u32 {
        unimplemented!()
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
    amount: u32,
    name: &str,
    products: &mut HashMap<String, u32>,
) -> u32 {
    let mut amount = amount;

    if name == "ORE" {
        return amount;
    }

    let existing = products.entry(name.to_string()).or_insert(0);

    if *existing >= amount {
        *existing -= amount;
        return 0;
    } else {
        amount -= *existing;
        *existing = 0;
    }

    let reaction = reactions.get(name).unwrap();
    let num_reactions = ((amount - 1) / reaction.output().amount()) + 1;

    let surplus = (num_reactions * reaction.output().amount()) - amount;
    *products.entry(name.to_string()).or_insert(0) += surplus;

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
        let problem = Day14 { input };

        assert_eq!(problem.part_one(), 31);
    }

    #[test]
    fn test_part_one_b() {
        let input = PathBuf::from("fixtures/day14b.txt");
        let problem = Day14 { input };

        assert_eq!(problem.part_one(), 165);
    }

    #[test]
    fn test_part_one_c() {
        let input = PathBuf::from("fixtures/day14c.txt");
        let problem = Day14 { input };

        assert_eq!(problem.part_one(), 13312);
    }

    #[test]
    fn test_part_one_d() {
        let input = PathBuf::from("fixtures/day14d.txt");
        let problem = Day14 { input };

        assert_eq!(problem.part_one(), 180697);
    }

    #[test]
    fn test_part_one_e() {
        let input = PathBuf::from("fixtures/day14e.txt");
        let problem = Day14 { input };

        assert_eq!(problem.part_one(), 2210736);
    }
}
