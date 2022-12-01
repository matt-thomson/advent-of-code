use std::{convert::Infallible, str::FromStr};

const BITS: [[usize; 4]; 16] = [
    [0, 0, 0, 0],
    [0, 0, 0, 1],
    [0, 0, 1, 0],
    [0, 0, 1, 1],
    [0, 1, 0, 0],
    [0, 1, 0, 1],
    [0, 1, 1, 0],
    [0, 1, 1, 1],
    [1, 0, 0, 0],
    [1, 0, 0, 1],
    [1, 0, 1, 0],
    [1, 0, 1, 1],
    [1, 1, 0, 0],
    [1, 1, 0, 1],
    [1, 1, 1, 0],
    [1, 1, 1, 1],
];

#[derive(Debug)]
pub struct BitStream {
    bits: Vec<usize>,
    position: usize,
}

impl FromStr for BitStream {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let bits = (0..input.len())
            .map(|i| usize::from_str_radix(&input[i..i + 1], 16).unwrap())
            .flat_map(|hex| BITS[hex])
            .collect();

        Ok(Self { bits, position: 0 })
    }
}

impl BitStream {
    pub fn read_decimal(&mut self, len: usize) -> usize {
        let result = self.bits[self.position..self.position + len]
            .iter()
            .fold(0, |acc, bit| acc * 2 + bit);

        self.position += len;

        result
    }

    pub fn read_bit(&mut self) -> usize {
        let result = self.bits[self.position];
        self.position += 1;

        result
    }

    pub fn substream(&mut self, len: usize) -> Self {
        let bits = self.bits[self.position..self.position + len].to_vec();
        self.position += len;

        Self { bits, position: 0 }
    }

    pub fn at_end(&self) -> bool {
        self.position == self.bits.len()
    }
}
