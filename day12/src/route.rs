use std::collections::BTreeSet;

pub trait Route: Default {
    fn position(&self) -> &str;
    fn step(&self, next: &str) -> Option<Self>;
}

#[derive(Debug)]
pub struct PartOneRoute {
    position: String,
    small_caves: BTreeSet<String>,
}

impl Default for PartOneRoute {
    fn default() -> Self {
        Self {
            position: "start".to_string(),
            small_caves: BTreeSet::new(),
        }
    }
}

impl Route for PartOneRoute {
    fn position(&self) -> &str {
        &self.position
    }

    fn step(&self, next: &str) -> Option<Self> {
        if self.small_caves.contains(next) {
            None
        } else {
            let mut small_caves = self.small_caves.clone();

            if next == next.to_lowercase() {
                small_caves.insert(next.to_string());
            }

            Some(Self {
                position: next.to_string(),
                small_caves,
            })
        }
    }
}

pub struct PartTwoRoute {
    position: String,
    small_caves: BTreeSet<String>,
    repeated_small_cave: Option<String>,
}

impl Default for PartTwoRoute {
    fn default() -> Self {
        Self {
            position: "start".to_string(),
            small_caves: BTreeSet::new(),
            repeated_small_cave: None,
        }
    }
}

impl Route for PartTwoRoute {
    fn position(&self) -> &str {
        &self.position
    }

    fn step(&self, next: &str) -> Option<Self> {
        if self.small_caves.contains(next) {
            if self.repeated_small_cave.is_some() {
                None
            } else {
                Some(Self {
                    position: next.to_string(),
                    small_caves: self.small_caves.clone(),
                    repeated_small_cave: Some(next.to_string()),
                })
            }
        } else {
            let mut small_caves = self.small_caves.clone();

            if next == next.to_lowercase() {
                small_caves.insert(next.to_string());
            }

            Some(Self {
                position: next.to_string(),
                small_caves,
                repeated_small_cave: self.repeated_small_cave.clone(),
            })
        }
    }
}
