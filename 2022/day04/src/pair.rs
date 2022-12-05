use std::{ops::RangeInclusive, str::FromStr};

use eyre::{eyre, ErrReport, Result};

#[derive(Debug)]
pub struct Pair {
    first: RangeInclusive<u64>,
    second: RangeInclusive<u64>,
}

impl FromStr for Pair {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (first, second) = input
            .split_once(',')
            .ok_or_else(|| eyre!("no comma in line"))?;

        Ok(Self {
            first: parse_range(first)?,
            second: parse_range(second)?,
        })
    }
}

impl Pair {
    pub fn fully_overlaps(&self) -> bool {
        contains(&self.first, &self.second) || contains(&self.second, &self.first)
    }

    pub fn partly_overlaps(&self) -> bool {
        let lower = self.first.start().max(self.second.start());
        let upper = self.first.end().min(self.second.end());

        lower <= upper
    }
}

fn parse_range(input: &str) -> Result<RangeInclusive<u64>> {
    let (lower, upper) = input
        .split_once('-')
        .ok_or_else(|| eyre!("no dash in range"))?;

    Ok(RangeInclusive::new(lower.parse()?, upper.parse()?))
}

fn contains(first: &RangeInclusive<u64>, second: &RangeInclusive<u64>) -> bool {
    first.start() <= second.start() && first.end() >= second.end()
}
