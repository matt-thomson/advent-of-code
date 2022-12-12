use eyre::Result;
use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::operation::Operation;

#[derive(Debug)]
pub struct Monkey {
    starting_items: Vec<u64>,
    operation: Operation,
    divisible_test: u64,
    throw_if_true: usize,
    throw_if_false: usize,
}

impl Monkey {
    pub fn parse_all(input: &str) -> Result<Vec<Self>> {
        todo!()
    }
}

fn monkey_identifier(input: &str) -> IResult<&str, usize> {
    delimited(
        tag("Monkey "),
        map(nom::character::complete::u16, usize::from),
        tag(":"),
    )(input)
}

fn starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u64),
    )(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    preceded(tag("Operation: "), Operation::parse)(input)
}

fn divisible_test(input: &str) -> IResult<&str, u64> {
    preceded(tag("Test: divisible by "), nom::character::complete::u64)(input)
}

fn throw_if_true(input: &str) -> IResult<&str, usize> {
    preceded(
        tag("If true: throw to monkey "),
        map(nom::character::complete::u16, usize::from),
    )(input)
}

fn throw_if_false(input: &str) -> IResult<&str, usize> {
    preceded(
        tag("If false: throw to monkey "),
        map(nom::character::complete::u16, usize::from),
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;

    #[test]
    fn test_monkey_identifier() {
        let input = "Monkey 3:";
        let (_, result) = monkey_identifier(input).finish().unwrap();

        assert_eq!(result, 3);
    }

    #[test]
    fn test_starting_items() {
        let input = "Starting items: 79, 98";
        let (_, result) = starting_items(input).finish().unwrap();

        assert_eq!(result, [79, 98]);
    }

    #[test]
    fn test_operation() {
        let input = "Operation: new = old * 19";
        let (_, result) = operation(input).finish().unwrap();

        assert_eq!(result, Operation::Multiply(19));
    }

    #[test]
    fn test_divisible_test() {
        let input = "Test: divisible by 23";
        let (_, result) = divisible_test(input).finish().unwrap();

        assert_eq!(result, 23);
    }

    #[test]
    fn test_throw_if_true() {
        let input = "If true: throw to monkey 2";
        let (_, result) = throw_if_true(input).finish().unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_throw_if_false() {
        let input = "If false: throw to monkey 3";
        let (_, result) = throw_if_false(input).finish().unwrap();

        assert_eq!(result, 3);
    }
}
