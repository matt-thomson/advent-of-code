mod fs_entry;
mod terminal_line;

use std::fs;
use std::path::Path;

use eyre::{eyre, Result};

use crate::fs_entry::FsEntry;
use crate::terminal_line::TerminalLine;

#[derive(Debug)]
pub struct Problem {
    root: FsEntry,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let terminal_lines = input.lines().map(str::parse).collect::<Result<Vec<_>>>()?;
        let root = process_lines(&terminal_lines)?;

        Ok(Self { root })
    }

    pub fn part_one(&self) -> Result<u64> {
        Ok(self
            .directory_sizes()
            .into_iter()
            .filter(|size| *size <= 100000)
            .sum())
    }

    pub fn part_two(&self) -> Result<u64> {
        let space_needed = self.root.size() - 40000000;

        self.directory_sizes()
            .into_iter()
            .filter(|size| *size >= space_needed)
            .min()
            .ok_or_else(|| eyre!("no directory big enough"))
    }

    fn directory_sizes(&self) -> Vec<u64> {
        self.root
            .directories()
            .iter()
            .map(|entry| entry.size())
            .collect()
    }
}

fn process_lines(lines: &[TerminalLine]) -> Result<FsEntry> {
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

    while let Some(entry) = entries.pop() {
        if let Some(parent) = entries.last_mut() {
            parent.add_child(entry)?;
        } else {
            return Ok(entry);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one().unwrap();

        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part_two() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_two().unwrap();

        assert_eq!(result, 24933642);
    }
}
