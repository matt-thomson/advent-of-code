use std::str::FromStr;

use eyre::{eyre, ErrReport};
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list0,
    sequence::delimited, Finish, IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    Number(u64),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, result) = Self::parse_packet(input)
            .finish()
            .map_err(|s| eyre!(s.to_string()))?;

        Ok(result)
    }
}

impl Packet {
    fn parse_number(input: &str) -> IResult<&str, Self> {
        map(nom::character::complete::u64, Packet::Number)(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("["),
                separated_list0(tag(","), Self::parse_packet),
                tag("]"),
            ),
            Packet::List,
        )(input)
    }

    fn parse_packet(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_number, Self::parse_list))(input)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Packet;

    #[rstest]
    #[case("123", Packet::Number(123))]
    #[case("[]", Packet::List(vec![]))]
    #[case("[123]", Packet::List(vec![Packet::Number(123)]))]
    #[case("[123,[456,789]]", Packet::List(vec![Packet::Number(123), Packet::List(vec![Packet::Number(456), Packet::Number(789)])]))]
    fn test_parse(#[case] input: String, #[case] expected: Packet) {
        let parsed: Packet = input.parse().unwrap();
        assert_eq!(parsed, expected);
    }
}
