use std::{convert::Infallible, str::FromStr};

use crate::bit_stream::bits;

#[derive(Debug)]
pub struct Packet {
    version: u32,
    _type_id: u32,
    value: u32,
}

impl FromStr for Packet {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut bits = bits(input).into_iter();

        let version = decimal(&mut bits.by_ref().take(3));
        let type_id = decimal(&mut bits.by_ref().take(3));

        let mut buffer = vec![];

        loop {
            let first = &bits.by_ref().next().unwrap();
            buffer.extend(bits.by_ref().take(4));

            if *first == 0 {
                break;
            }
        }

        let value = decimal(&mut buffer.into_iter());

        Ok(Self {
            version,
            _type_id: type_id,
            value,
        })
    }
}

fn decimal<I>(bits: &mut I) -> u32
where
    I: Iterator<Item = u32>,
{
    bits.fold(0, |acc, bit| acc * 2 + bit)
}
