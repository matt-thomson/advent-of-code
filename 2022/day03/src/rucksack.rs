use std::collections::BTreeSet;
use std::convert::Infallible;
use std::str::FromStr;

use eyre::{eyre, Result};

use crate::item::Item;

#[derive(Debug)]
pub struct Rucksack {
    first: Vec<Item>,
    second: Vec<Item>,
}

impl FromStr for Rucksack {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let items: Vec<_> = input.chars().map(Item::from).collect();
        let (first, second) = items.split_at(items.len() / 2);

        Ok(Self {
            first: first.to_vec(),
            second: second.to_vec(),
        })
    }
}

impl Rucksack {
    pub fn duplicate(&self) -> Result<&Item> {
        let first = BTreeSet::from_iter(self.first.iter());
        let second = BTreeSet::from_iter(self.second.iter());

        let mut intersection = first.intersection(&second);

        intersection
            .next()
            .ok_or_else(|| eyre!("no item in both compartments"))
            .copied()
    }
}
