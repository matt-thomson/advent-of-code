use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Problem {
    fish: [u64; 9],
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut fish = [0u64; 9];

        fs::read_to_string(&path)
            .unwrap()
            .trim()
            .split(',')
            .map(|age| age.parse().unwrap())
            .for_each(|age: usize| fish[age] += 1);

        Self { fish }
    }

    pub fn part_one(&self) -> u64 {
        self.solve(80)
    }

    pub fn part_two(&self) -> u64 {
        self.solve(256)
    }

    fn solve(&self, days: usize) -> u64 {
        let fish = (0..days).fold(self.fish, |current, _| next_day(&current));
        fish.iter().sum()
    }
}

fn next_day(current: &[u64; 9]) -> [u64; 9] {
    let mut result = [0u64; 9];

    result[..8].clone_from_slice(&current[1..]);

    result[6] += current[0];
    result[8] = current[0];

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 5934);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 26984457539);
    }
}
