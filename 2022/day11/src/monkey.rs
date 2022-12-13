use eyre::{eyre, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space0},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
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
        let (_, monkeys) = separated_list1(line_ending, Self::parse)(input)
            .finish()
            .map_err(|s| eyre!(s.to_string()))?;

        Ok(monkeys)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                terminated(monkey_identifier, line_ending),
                preceded(space0, terminated(starting_items, line_ending)),
                preceded(space0, terminated(operation, line_ending)),
                preceded(space0, terminated(divisible_test, line_ending)),
                preceded(space0, terminated(throw_if_true, line_ending)),
                preceded(space0, terminated(throw_if_false, line_ending)),
            )),
            |(_, starting_items, operation, divisible_test, throw_if_true, throw_if_false)| {
                Monkey {
                    starting_items,
                    operation,
                    divisible_test,
                    throw_if_true,
                    throw_if_false,
                }
            },
        )(input)
    }

    pub fn inspect(&self, worry_level: u64, divisor: u64, modulus: u64) -> (u64, usize) {
        let worry_level = (self.operation.apply(worry_level) / divisor) % modulus;

        let destination = if worry_level % self.divisible_test == 0 {
            self.throw_if_true
        } else {
            self.throw_if_false
        };

        (worry_level, destination)
    }

    pub fn starting_items(&self) -> &[u64] {
        &self.starting_items
    }

    pub fn divisible_test(&self) -> u64 {
        self.divisible_test
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
    fn test_parse_monkey() {
        let input = "Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
        ";

        let (_, monkey) = Monkey::parse(input).finish().unwrap();

        assert_eq!(monkey.starting_items, [79, 98]);
        assert_eq!(monkey.operation, Operation::Multiply(19));
        assert_eq!(monkey.divisible_test, 23);
        assert_eq!(monkey.throw_if_true, 2);
        assert_eq!(monkey.throw_if_false, 3);
    }
}
