mod bit_stream;
mod packet;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use packet::Packet;

#[derive(Debug)]
pub struct Problem {
    packets: Vec<Packet>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let packets: Vec<_> = reader
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

        Self { packets }
    }

    pub fn part_one(&self) -> usize {
        dbg!(self);

        self.packets.iter().map(|packet| packet.version_sum()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt");

        assert_eq!(problem.part_one(), 29);
    }
}
