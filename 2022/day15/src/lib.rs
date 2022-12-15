mod point;
mod reading;

use std::fs;
use std::path::Path;

use eyre::Result;

use crate::reading::Reading;

#[derive(Debug)]
pub struct Problem {
    readings: Vec<Reading>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let readings: Result<Vec<Reading>> = input.lines().map(|line| line.parse()).collect();

        Ok(Self {
            readings: readings?,
        })
    }

    pub fn part_one(&self) -> usize {
        dbg!(self);
        todo!()
    }
}
