use std::str::FromStr;

use eyre::{eyre, ErrReport};
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::separated_pair,
    Finish, IResult,
};

#[derive(Debug)]
pub struct Rock {
    points: Vec<(usize, usize)>,
}

impl FromStr for Rock {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, points) = separated_list1(tag(" -> "), parse_pair)(input)
            .finish()
            .map_err(|s| eyre!(s.to_string()))?;

        Ok(Self { points })
    }
}

fn parse_pair(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(
        map(nom::character::complete::u16, usize::from),
        tag(","),
        map(nom::character::complete::u16, usize::from),
    )(input)
}
