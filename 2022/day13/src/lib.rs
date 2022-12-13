mod packet;

use std::fs;
use std::path::Path;

use eyre::Result;

use crate::packet::Packet;

#[derive(Debug)]
pub struct Problem {
    packet_pairs: Vec<(Packet, Packet)>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let lines: Vec<_> = input.lines().collect();
        let packet_pairs: Result<Vec<(Packet, Packet)>> = lines
            .chunks(3)
            .map(|chunk| Ok((chunk[0].parse()?, chunk[1].parse()?)))
            .collect();

        Ok(Self {
            packet_pairs: packet_pairs?,
        })
    }

    pub fn part_one(&self) -> usize {
        self.packet_pairs
            .iter()
            .enumerate()
            .filter(|(_, (first, second))| first < second)
            .map(|(index, _)| index + 1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one();

        assert_eq!(result, 13);
    }
}
