mod fs_entry;
mod terminal_line;

use std::fs;
use std::path::Path;

use eyre::{eyre, Result};

use crate::fs_entry::FsEntry;
use crate::terminal_line::TerminalLine;

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

    pub fn part_one(&self) -> Result<u64> {
        let entries = process_lines(&mut self.terminal_lines.iter())?;
        dbg!(entries);

        todo!()
    }
}

fn process_lines<'a, I: Iterator<Item = &'a TerminalLine>>(lines: &mut I) -> Result<Vec<FsEntry>> {
    let mut entries = vec![FsEntry::Directory {
        name: "/".to_string(),
        children: vec![],
    }];

    for line in lines {
        match line {
            TerminalLine::List => {}
            TerminalLine::ChangeRoot => {}
            TerminalLine::ChangeOut => {
                let current = entries.pop().unwrap();
                entries
                    .last_mut()
                    .ok_or_else(|| eyre!("tried to change out from top level"))?
                    .add_child(current)?;
            }
            TerminalLine::ChangeIn { name } => {
                entries.push(FsEntry::Directory {
                    name: name.to_string(),
                    children: vec![],
                });
            }
            TerminalLine::File { name, size } => {
                entries
                    .last_mut()
                    .ok_or_else(|| eyre!("tried to change out from top level"))?
                    .add_child(FsEntry::File {
                        name: name.to_string(),
                        size: *size,
                    })?;
            }
            TerminalLine::Directory { name: _name } => {}
        }
    }

    Ok(entries)
}
