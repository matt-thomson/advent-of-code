use std::{convert::Infallible, str::FromStr};

use crate::bit_stream::bits;

#[derive(Debug)]
pub struct Packet {
    version: usize,
    content: Content,
}

#[derive(Debug)]
pub enum Content {
    Literal(usize),
    Operator(usize, Vec<Packet>),
}

impl FromStr for Packet {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut bits = bits(input).into_iter();

        Ok(read_packet(&mut bits))
    }
}

impl Packet {
    pub fn version_sum(&self) -> usize {
        match &self.content {
            Content::Literal(_) => self.version,
            Content::Operator(_, subpackets) => {
                self.version
                    + subpackets
                        .iter()
                        .map(|packet| packet.version_sum())
                        .sum::<usize>()
            }
        }
    }
}

fn read_packet<I>(bits: &mut I) -> Packet
where
    I: Iterator<Item = usize>,
{
    let version = decimal(&mut bits.by_ref().take(3));
    let type_id = decimal(&mut bits.by_ref().take(3));

    if type_id == 4 {
        let value = read_literal(&mut bits.by_ref());

        Packet {
            version,
            content: Content::Literal(value),
        }
    } else {
        if bits.next().unwrap() == 0 {
            let length = decimal(&mut bits.by_ref().take(15));
            let rest: Vec<_> = bits.take(length).collect();

            let mut content = rest.into_iter().peekable();
            let mut packets = vec![];

            while content.peek().is_some() {
                packets.push(read_packet(&mut content.by_ref()));
            }

            Packet {
                version,
                content: Content::Operator(type_id, packets),
            }
        } else {
            unreachable!();
        }
    }
}

fn read_literal<I>(bits: &mut I) -> usize
where
    I: Iterator<Item = usize>,
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

fn decimal<I>(bits: &mut I) -> usize
where
    I: Iterator<Item = usize>,
{
    bits.fold(0, |acc, bit| acc * 2 + bit)
}
