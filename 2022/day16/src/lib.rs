mod valve;

use std::fs;
use std::path::Path;

use eyre::Result;

use crate::valve::Valve;

#[derive(Debug)]
pub struct Problem {
    valves: Vec<Valve>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let valves: Result<Vec<Valve>> = input.lines().map(str::parse).collect();

        Ok(Self { valves: valves? })
    }

    pub fn part_one(&self) -> usize {
        dbg!(self);
        todo!()
    }
}
