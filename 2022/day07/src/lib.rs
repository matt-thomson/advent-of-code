mod terminal_line;

use std::fs;
use std::path::Path;

use eyre::Result;
use terminal_line::TerminalLine;

#[derive(Debug)]
pub struct Problem {
    terminal_lines: Vec<TerminalLine>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let terminal_lines = input.lines().map(str::parse).collect::<Result<Vec<_>>>()?;

        Ok(Self { terminal_lines })
    }

    pub fn part_one(&self) -> u64 {
        dbg!(self);
        todo!()
    }
}
