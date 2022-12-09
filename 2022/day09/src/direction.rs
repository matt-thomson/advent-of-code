use std::str::FromStr;

use eyre::{eyre, ErrReport};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ErrReport;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(eyre!("invalid direction: {input}")),
        }
    }
}
