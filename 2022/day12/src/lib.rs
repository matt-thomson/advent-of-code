mod heightmap;

use std::fs;
use std::path::Path;

use eyre::Result;

use crate::heightmap::Heightmap;

#[derive(Debug)]
pub struct Problem {
    heightmap: Heightmap,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let heightmap = input.parse()?;

        Ok(Self { heightmap })
    }

    pub fn part_one(&self) -> usize {
        dbg!(self);
        todo!()
    }
}
