use std::{convert::Infallible, str::FromStr};

use crate::bit_stream::bits;

#[derive(Debug)]
pub struct Packet {
    version: u32,
    _content: Content,
}

#[derive(Debug)]
pub enum Content {
    Literal(u32),
    Operator(u32, Vec<Packet>),
}

impl FromStr for Packet {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut bits = bits(input).into_iter();

        let version = decimal(&mut bits.by_ref().take(3));
        let type_id = decimal(&mut bits.by_ref().take(3));

        if type_id == 4 {
            let value = read_literal(&mut bits.by_ref());

            return Ok(Self {
                version,
                _content: Content::Literal(value),
            });
        }

        unreachable!()
    }
}

impl Packet {
    pub fn version_sum(&self) -> u32 {
        self.version
    }
}

fn read_literal<I>(bits: &mut I) -> u32
where
    I: Iterator<Item = u32>,
{
    let mut buffer = vec![];

    loop {
        let first = bits.by_ref().next().unwrap();
        buffer.extend(bits.by_ref().take(4));

        if first == 0 {
            break;
        }
    }

    decimal(&mut buffer.into_iter())
}

fn decimal<I>(bits: &mut I) -> u32
where
    I: Iterator<Item = u32>,
{
    bits.fold(0, |acc, bit| acc * 2 + bit)
}
