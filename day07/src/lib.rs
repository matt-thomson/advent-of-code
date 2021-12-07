use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Problem {
    crabs: Vec<u32>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let crabs = fs::read_to_string(&path)
            .unwrap()
            .trim()
            .split(',')
            .map(|crab| crab.parse().unwrap())
            .collect();

        Self { crabs }
    }

    pub fn part_one(&self) -> u32 {
        self.solve(&|x| x)
    }

    pub fn part_two(&self) -> u32 {
        self.solve(&|x| (x * (x + 1)) / 2)
    }

    fn solve<F>(&self, burn_rate: &F) -> u32
    where
        F: Fn(u32) -> u32,
    {
        (*self.crabs.iter().min().unwrap()..=*self.crabs.iter().max().unwrap())
            .map(|crab| self.fuel_required(crab, &burn_rate))
            .min()
            .unwrap()
    }

    fn fuel_required<F>(&self, destination: u32, burn_rate: &F) -> u32
    where
        F: Fn(u32) -> u32,
    {
        self.crabs
            .iter()
            .map(|&crab| abs_diff(crab, destination))
            .map(burn_rate)
            .sum()
    }
}

fn abs_diff(x: u32, y: u32) -> u32 {
    if x > y {
        x - y
    } else {
        y - x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 37);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 168);
    }
}
