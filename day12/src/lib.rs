mod network;
mod route;

use std::fs;
use std::path::Path;

use network::Network;
use route::{PartOneRoute, PartTwoRoute};

#[derive(Debug)]
pub struct Problem {
    network: Network,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let network: Network = fs::read_to_string(&path).unwrap().parse().unwrap();

        Self { network }
    }

    pub fn part_one(&self) -> usize {
        self.network.find_routes::<PartOneRoute>()
    }

    pub fn part_two(&self) -> usize {
        self.network.find_routes::<PartTwoRoute>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 226);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 3509);
    }
}
