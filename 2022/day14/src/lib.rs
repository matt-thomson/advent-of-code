mod cave;
mod rock;
mod segment;

use std::fs;
use std::path::Path;

use eyre::Result;

use crate::cave::Cave;
use crate::rock::Rock;

#[derive(Debug)]
pub struct Problem {
    rocks: Vec<Rock>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let rocks: Result<Vec<Rock>> = input.lines().map(|line| line.parse()).collect();

        Ok(Self { rocks: rocks? })
    }

    pub fn part_one(&self) -> Result<usize> {
        let cave = Cave::new(&self.rocks)?;
        dbg!(cave);

        todo!();
    }
}
