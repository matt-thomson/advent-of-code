use std::str::FromStr;

use strum::EnumString;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    length: u32,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.trim().split(' ');

        let direction: Direction = parts.next().unwrap().parse()?;
        let length: u32 = parts.next().unwrap().parse()?;

        Ok(Self { direction, length })
    }
}

impl Instruction {
    pub fn step_part_one(&self, (position, depth): (u32, u32)) -> (u32, u32) {
        match self.direction {
            Direction::Forward => (position + self.length, depth),
            Direction::Down => (position, depth + self.length),
            Direction::Up => (position, depth - self.length),
        }
    }

    pub fn step_part_two(&self, (position, depth, aim): (u32, u32, u32)) -> (u32, u32, u32) {
        match self.direction {
            Direction::Forward => (position + self.length, depth + aim * self.length, aim),
            Direction::Down => (position, depth, aim + self.length),
            Direction::Up => (position, depth, aim - self.length),
        }
    }
}
