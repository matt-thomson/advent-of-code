use std::collections::BTreeSet;

use eyre::{eyre, Result};

use crate::item::Item;

#[derive(Debug)]
pub struct Rucksack {
    groups: Vec<BTreeSet<Item>>,
}

impl Rucksack {
    pub fn new<T: AsRef<str>>(input: &[T]) -> Self {
        let groups = input
            .iter()
            .map(|group| group.as_ref().chars().map(Item::from).collect())
            .collect();

        Self { groups }
    }

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
