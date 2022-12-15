use std::str::FromStr;

use eyre::{eyre, ErrReport};
use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

use crate::point::Point;

#[derive(Debug)]
pub struct Reading {
    sensor: Point,
    beacon: Point,
}

impl FromStr for Reading {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, reading) = map(
            separated_pair(
                preceded(tag("Sensor at "), parse_point),
                tag(": closest beacon is at "),
                parse_point,
            ),
            |(sensor, beacon)| Self { sensor, beacon },
        )(input)
        .finish()
        .map_err(|s| eyre!(s.to_string()))?;

        Ok(reading)
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            preceded(tag("x="), nom::character::complete::i64),
            tag(", y="),
            nom::character::complete::i64,
        ),
        |(x, y)| Point::new(x, y),
    )(input)
}

impl Reading {
    pub fn sensor(&self) -> &Point {
        &self.sensor
    }

    pub fn beacon(&self) -> &Point {
        &self.beacon
    }
}

#[cfg(test)]
mod tests {
    use super::Reading;

    #[test]
    fn test_parse() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let reading: Reading = input.parse().unwrap();

        assert_eq!(reading.sensor().x(), 2);
        assert_eq!(reading.sensor().y(), 18);
        assert_eq!(reading.beacon().x(), -2);
        assert_eq!(reading.beacon().y(), 15);
    }
}
