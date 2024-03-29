use std::fmt::{Debug, Formatter};

use eyre::{eyre, Result};

pub enum FsEntry {
    File {
        name: String,
        size: u64,
    },
    Directory {
        name: String,
        children: Vec<FsEntry>,
    },
}

impl Debug for FsEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print_tree(f, 0)
    }
}

impl FsEntry {
    pub fn add_child(&mut self, entry: FsEntry) -> Result<()> {
        match self {
            FsEntry::File { .. } => Err(eyre!("can't add child to file")),
            FsEntry::Directory { children, .. } => {
                children.push(entry);
                Ok(())
            }
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            FsEntry::File { size, .. } => *size,
            FsEntry::Directory { children, .. } => children.iter().map(FsEntry::size).sum(),
        }
    }

    pub fn directories(&self) -> Vec<&FsEntry> {
        match self {
            FsEntry::File { .. } => vec![],
            FsEntry::Directory { children, .. } => {
                let mut result = vec![self];

                for child in children {
                    result.extend(child.directories());
                }

                result
            }
        }
    }

    fn print_tree(&self, f: &mut Formatter<'_>, indent: usize) -> std::fmt::Result {
        match self {
            FsEntry::File { name, size } => {
                writeln!(f, "{:indent$}- {name} (file, size={size})", ' ')
            }
            FsEntry::Directory { name, children } => {
                writeln!(f, "{:indent$}- {name} (dir)", ' ')?;
                for child in children {
                    child.print_tree(f, indent + 2)?;
                }

                Ok(())
            }
        }
    }
}
