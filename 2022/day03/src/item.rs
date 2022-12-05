use eyre::{eyre, Result};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item(char);

impl From<char> for Item {
    fn from(c: char) -> Self {
        Self(c)
    }
}

impl Item {
    pub fn priority(&self) -> Result<u64> {
        match self.0 {
            'A'..='Z' => Ok((self.0 as u64) - 65 + 27),
            'a'..='z' => Ok((self.0 as u64) - 97 + 1),
            _ => Err(eyre!("invalid item {}", self.0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_values() {
        assert_eq!(Item::from('p').priority().unwrap(), 16);
        assert_eq!(Item::from('L').priority().unwrap(), 38);
    }
}
