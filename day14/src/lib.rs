use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Problem {
    template: HashMap<[char; 2], u64>,
    rules: HashMap<[char; 2], char>,
    ends: [char; 2],
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let mut lines = BufReader::new(file).lines();

        let mut template = HashMap::new();
        let polymers: Vec<_> = lines.next().unwrap().unwrap().chars().collect();
        polymers
            .windows(2)
            .for_each(|pair| *template.entry([pair[0], pair[1]]).or_insert(0) += 1);

        let rules = lines
            .skip(1)
            .map(|line| line.unwrap().chars().collect::<Vec<_>>())
            .map(|chars| (chars[0..2].try_into().unwrap(), chars[6]))
            .collect();

        Self {
            template,
            rules,
            ends: [*polymers.first().unwrap(), *polymers.last().unwrap()],
        }
    }

    pub fn part_one(&self) -> u64 {
        self.run(10)
    }

    pub fn part_two(&self) -> u64 {
        self.run(40)
    }

    fn run(&self, steps: usize) -> u64 {
        let mut pairs = (0..steps).fold(self.template.clone(), |pairs, _| self.step(&pairs));
        *pairs.entry(self.ends).or_insert(0) += 1;

        let mut counts = HashMap::new();

        for (&pair, &count) in pairs.iter() {
            pair.iter()
                .for_each(|polymer| *counts.entry(*polymer).or_insert(0) += count);
        }

        (counts.values().max().unwrap() - counts.values().min().unwrap()) / 2
    }

    fn step(&self, pairs: &HashMap<[char; 2], u64>) -> HashMap<[char; 2], u64> {
        let mut result = HashMap::new();

        for (&pair, &count) in pairs {
            let new = *self.rules.get(&pair).unwrap();

            [[pair[0], new], [new, pair[1]]]
                .iter()
                .for_each(|pair| *result.entry(*pair).or_insert(0) += count);
        }

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

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 2188189693529);
    }
}
