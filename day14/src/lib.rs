use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Problem {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let mut lines = BufReader::new(file).lines();

        let template = lines.next().unwrap().unwrap().chars().collect();

        let rules = lines
            .skip(1)
            .map(|line| line.unwrap().chars().collect::<Vec<_>>())
            .map(|chars| ((chars[0], chars[1]), chars[6]))
            .collect();

        Self { template, rules }
    }

    pub fn part_one(&self) -> u32 {
        let polymers = (0..10).fold(self.template.clone(), |polymers, _| self.step(&polymers));
        let mut counts = HashMap::new();

        polymers
            .iter()
            .for_each(|polymer| *counts.entry(polymer).or_insert(0) += 1);

        counts.values().max().unwrap() - counts.values().min().unwrap()
    }

    fn step(&self, polymers: &[char]) -> Vec<char> {
        let mut result: Vec<_> = polymers
            .windows(2)
            .flat_map(|pair| [pair[0], *self.rules.get(&(pair[0], pair[1])).unwrap()])
            .collect();

        result.push(*polymers.last().unwrap());

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 1588);
    }
}
