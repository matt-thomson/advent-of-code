use std::str::FromStr;

use eyre::{eyre, ErrReport};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::preceded,
    Finish,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    NoOp,
    Add(i64),
}

impl FromStr for Instruction {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parser = alt((
            value(Self::NoOp, tag("noop")),
            map(
                preceded(tag("addx "), nom::character::complete::i64),
                |value| Self::Add(value),
            ),
        ));

        let (_, result) = parser(input)
            .finish()
            .map_err(|s: nom::error::Error<&str>| eyre!(s.to_string()))?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Instruction;

    #[rstest]
    #[case("noop", Instruction::NoOp)]
    #[case("addx 123", Instruction::Add(123))]
    #[case("addx -456", Instruction::Add(-456))]
    fn test_parse(#[case] input: String, #[case] expected: Instruction) {
        let parsed: Instruction = input.parse().unwrap();
        assert_eq!(parsed, expected);
    }
}
