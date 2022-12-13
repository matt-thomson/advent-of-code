use std::{cmp::Ordering, str::FromStr};

use eyre::{eyre, ErrReport};
use itertools::{EitherOrBoth, Itertools};
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list0,
    sequence::delimited, Finish, IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(first), Packet::Number(second)) => first.cmp(second),
            (Packet::Number(number), list @ Packet::List(_)) => {
                Packet::List(vec![Packet::Number(*number)]).cmp(list)
            }
            (list @ Packet::List(_), Packet::Number(number)) => {
                list.cmp(&Packet::List(vec![Packet::Number(*number)]))
            }
            (Packet::List(first), Packet::List(second)) => {
                for pair in first.iter().zip_longest(second) {
                    match pair {
                        EitherOrBoth::Both(first, second) => {
                            let ordering = first.cmp(second);
                            if ordering.is_ne() {
                                return ordering;
                            }
                        }
                        EitherOrBoth::Left(_) => return Ordering::Greater,
                        EitherOrBoth::Right(_) => return Ordering::Less,
                    }
                }

                Ordering::Equal
            }
        }
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

    #[rstest]
    #[case("[1,1,3,1,1]", "[1,1,5,1,1]")]
    #[case("[[1],[2,3,4]]", "[[1],4]")]
    #[case("[[8,7,6]]", "[9]")]
    #[case("[[4,4],4,4]", "[[4,4],4,4,4]")]
    #[case("[7,7,7]", "[7,7,7,7]")]
    #[case("[]", "[3]")]
    #[case("[[]]", "[[[]]]")]
    #[case("[1,[2,[3,[4,[5,6,0]]]],8,9]", "[1,[2,[3,[4,[5,6,7]]]],8,9]")]
    fn test_ordering(#[case] first: Packet, #[case] second: Packet) {
        assert!(first < second);
        assert!(second > first);
    }
}
