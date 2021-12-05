use std::{num::ParseIntError, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    pub fn points(&self) -> Vec<(u32, u32)> {
        if self.x1 == self.x2 {
            let start = self.y1.min(self.y2);
            let end = self.y1.max(self.y2);

            (start..=end).map(|y| (self.x1, y)).collect()
        } else if self.y1 == self.y2 {
            let start = self.x1.min(self.x2);
            let end = self.x1.max(self.x2);

            (start..=end).map(|x| (x, self.y1)).collect()
        } else {
            let x_range: Vec<u32> = if self.x1 < self.x2 {
                (self.x1..=self.x2).collect()
            } else {
                (self.x2..=self.x1).rev().collect()
            };

            let y_range: Vec<u32> = if self.y1 < self.y2 {
                (self.y1..=self.y2).collect()
            } else {
                (self.y2..=self.y1).rev().collect()
            };

            x_range.into_iter().zip(y_range.into_iter()).collect()
        }
    }

    pub fn is_diagonal(&self) -> bool {
        self.x1 != self.x2 && self.y1 != self.y2
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").unwrap();
        }

        let captures = RE.captures(input).unwrap();

        let mut numbers = captures
            .iter()
            .skip(1)
            .map(|capture| capture.unwrap().as_str().parse());

        let x1 = numbers.next().unwrap()?;
        let y1 = numbers.next().unwrap()?;
        let x2 = numbers.next().unwrap()?;
        let y2 = numbers.next().unwrap()?;

        Ok(Line { x1, y1, x2, y2 })
    }
}
