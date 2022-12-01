mod bit_stream;
mod packet;

use std::fs;
use std::path::Path;

use packet::Packet;

#[derive(Debug)]
pub struct Problem {
    packet: Packet,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let input = fs::read_to_string(path).unwrap();
        let packet = input.trim().parse().unwrap();

        Self { packet }
    }

    pub fn part_one(&self) -> usize {
        self.packet.version_sum()
    }

    pub fn part_two(&self) -> usize {
        self.packet.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 20);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_two(), 1);
    }
}
