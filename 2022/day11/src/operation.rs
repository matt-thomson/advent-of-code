use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Operation::Square, tag("new = old * old")),
            map(
                preceded(tag("new = old + "), nom::character::complete::u64),
                Operation::Add,
            ),
            map(
                preceded(tag("new = old * "), nom::character::complete::u64),
                Operation::Multiply,
            ),
        ))(input)
    }

    pub fn apply(&self, worry_level: u64) -> u64 {
        match self {
            Operation::Add(other) => worry_level + other,
            Operation::Multiply(other) => worry_level * other,
            Operation::Square => worry_level * worry_level,
        }
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;
    use rstest::rstest;

    use super::Operation;

    #[rstest]
    #[case("new = old * old", Operation::Square)]
    #[case("new = old + 123", Operation::Add(123))]
    #[case("new = old * 456", Operation::Multiply(456))]
    fn test_parse(#[case] input: &str, #[case] expected: Operation) {
        let (_, parsed) = Operation::parse(input).finish().unwrap();
        assert_eq!(parsed, expected);
    }
}
