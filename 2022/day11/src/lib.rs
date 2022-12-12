mod monkey;

use std::fs;
use std::path::Path;

use eyre::Result;

use crate::monkey::Monkey;

#[derive(Debug)]
pub struct Problem {
    monkeys: Vec<Monkey>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let monkeys = Monkey::parse_all(&input)?;

        Ok(Self { monkeys })
    }

    pub fn part_one(&self) -> usize {
        dbg!(self);

        todo!()
    }
}
