use std::convert::Infallible;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::position::Position;

#[derive(Debug)]
pub struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl FromStr for Target {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("target area: x=(?P<x_min>-?\\d+)..(?P<x_max>-?\\d+), y=(?P<y_min>-?\\d+)..(?P<y_max>-?\\d+)").unwrap();
        }

        let captures = RE.captures(input).unwrap();

        let x_min = captures.name("x_min").unwrap().as_str().parse().unwrap();
        let x_max = captures.name("x_max").unwrap().as_str().parse().unwrap();
        let y_min = captures.name("y_min").unwrap().as_str().parse().unwrap();
        let y_max = captures.name("y_max").unwrap().as_str().parse().unwrap();

        Ok(Self {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

impl Target {
    pub fn contains(&self, position: &Position) -> bool {
        (self.x_min..=self.x_max).contains(&position.x)
            && (self.y_min..=self.y_max).contains(&position.y)
    }
}
