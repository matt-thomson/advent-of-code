mod stacks;

use std::fs;
use std::path::Path;

use eyre::{eyre, Result};

use crate::stacks::Stacks;

#[derive(Debug)]
pub struct Problem {
    stacks: Stacks,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let (stacks, moves) = input
            .split_once("\n\n")
            .ok_or_else(|| eyre!("couldn't split into stacks and moves"))?;

        Ok(Self {
            stacks: stacks.parse()?,
        })
    }

    pub fn part_one(&self) -> String {
        dbg!(self);
        todo!()
    }
}
