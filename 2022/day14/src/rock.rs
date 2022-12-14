use std::str::FromStr;

use eyre::{eyre, ErrReport};
use nom::{
    bytes::complete::tag, multi::separated_list1, sequence::separated_pair, Finish, IResult,
};

#[derive(Debug)]
pub struct Rock {
    points: Vec<(u16, u16)>,
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

fn parse_pair(input: &str) -> IResult<&str, (u16, u16)> {
    separated_pair(
        nom::character::complete::u16,
        tag(","),
        nom::character::complete::u16,
    )(input)
}
