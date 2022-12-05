use std::collections::BTreeSet;
use std::convert::Infallible;
use std::str::FromStr;

use eyre::{eyre, Result};

use crate::item::Item;

#[derive(Debug)]
pub struct Rucksack {
    groups: Vec<BTreeSet<Item>>,
}

impl FromStr for Rucksack {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let items: Vec<_> = input.chars().map(Item::from).collect();
        let (first, second) = items.split_at(items.len() / 2);

        Ok(Self {
            groups: vec![
                BTreeSet::from_iter(first.iter().cloned()),
                BTreeSet::from_iter(second.iter().cloned()),
            ],
        })
    }
}

impl Rucksack {
    pub fn duplicate(&self) -> Result<Item> {
        let mut intersection = self.groups[0].clone();

        for group in &self.groups[1..] {
            intersection = intersection.intersection(group).cloned().collect();
        }

        intersection
            .iter()
            .next()
            .ok_or_else(|| eyre!("no item in all compartments"))
            .cloned()
    }
}
