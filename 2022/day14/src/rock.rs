use std::str::FromStr;

use eyre::{eyre, ErrReport, Result};
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::separated_pair,
    Finish, IResult,
};

use crate::segment::Segment;

#[derive(Debug)]
pub struct Rock {
    corners: Vec<(usize, usize)>,
}

impl FromStr for Rock {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, corners) = separated_list1(tag(" -> "), parse_pair)(input)
            .finish()
            .map_err(|s| eyre!(s.to_string()))?;

        Ok(Self { corners })
    }
}

fn parse_pair(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(
        map(nom::character::complete::u16, usize::from),
        tag(","),
        map(nom::character::complete::u16, usize::from),
    )(input)
}

impl Rock {
    pub fn points(&self) -> Result<Vec<(usize, usize)>> {
        Ok(self
            .corners
            .windows(2)
            .map(|pair| Segment::new(pair[0], pair[1]).points())
            .collect::<Result<Vec<Vec<(usize, usize)>>>>()
            .map(|points| points.into_iter().flatten())?
            .collect())
    }
}
