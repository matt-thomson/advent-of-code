use std::convert::Infallible;
use std::str::FromStr;

use crate::bit_stream::BitStream;

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
        let mut bit_stream = input.parse().unwrap();

        Ok(read_packet(&mut bit_stream))
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

fn read_packet(bit_stream: &mut BitStream) -> Packet {
    let version = bit_stream.read_decimal(3);
    let type_id = bit_stream.read_decimal(3);

    if type_id == 4 {
        let value = read_literal(bit_stream);

        Packet {
            version,
            content: Content::Literal(value),
        }
    } else {
        match bit_stream.read_bit() {
            0 => {
                let length = bit_stream.read_decimal(15);
                let mut rest = bit_stream.substream(length);
                let mut packets = vec![];

                while !rest.at_end() {
                    packets.push(read_packet(&mut rest));
                }

                Packet {
                    version,
                    content: Content::Operator(type_id, packets),
                }
            }
            1 => {
                let num_packets = bit_stream.read_decimal(11);
                let mut packets = vec![];

                for _ in 0..num_packets {
                    packets.push(read_packet(bit_stream));
                }

                Packet {
                    version,
                    content: Content::Operator(type_id, packets),
                }
            }
            _ => unreachable!(),
        }
    }
}

fn read_literal(bit_stream: &mut BitStream) -> usize {
    let mut result = 0;

    loop {
        let first = bit_stream.read_bit();
        result = result * 16 + bit_stream.read_decimal(4);

        if first == 0 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("D2FE28", 6)]
    #[case("38006F45291200", 9)]
    #[case("EE00D40C823060", 14)]
    #[case("8A004A801A8002F478", 16)]
    #[case("620080001611562C8802118E34", 12)]
    #[case("C0015000016115A2E0802F182340", 23)]
    #[case("A0016C880162017C3686B18A3D4780", 31)]
    fn test_version_sum(#[case] input: &str, #[case] expected: usize) {
        let packet: Packet = input.parse().unwrap();
        assert_eq!(packet.version_sum(), expected);
    }
}
