use eyre::{eyre, Result};

#[derive(Debug)]
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
}
