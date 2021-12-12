use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Route {
    position: String,
    small_caves: BTreeSet<String>,
    can_repeat: bool,
}

impl Route {
    pub fn new(can_repeat: bool) -> Self {
        Self {
            position: "start".to_string(),
            small_caves: BTreeSet::new(),
            can_repeat,
        }
    }

    pub fn position(&self) -> &str {
        &self.position
    }

    pub fn step(&self, next: &str) -> Option<Self> {
        if self.small_caves.contains(next) {
            if !self.can_repeat {
                None
            } else {
                Some(Self {
                    position: next.to_string(),
                    small_caves: self.small_caves.clone(),
                    can_repeat: false,
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
                can_repeat: self.can_repeat,
            })
        }
    }
}
