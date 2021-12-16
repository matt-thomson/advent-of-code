use std::{convert::Infallible, str::FromStr};

use crate::bit_stream::bits;

#[derive(Debug)]
pub struct Packet {
    version: u32,
    _type_id: u32,
}

impl FromStr for Packet {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let bits = bits(input);

        let version = decimal(&bits[0..3]);
        let type_id = decimal(&bits[3..6]);

        Ok(Self {
            version,
            _type_id: type_id,
        })
    }
}

fn decimal(bits: &[u32]) -> u32 {
    bits.iter().fold(0, |acc, bit| acc * 2 + bit)
}
