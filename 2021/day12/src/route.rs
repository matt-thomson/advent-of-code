use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Route<'a> {
    position: &'a str,
    small_caves: BTreeSet<&'a str>,
    can_repeat: bool,
}

impl<'a> Route<'a> {
    pub fn new(can_repeat: bool) -> Self {
        Self {
            position: "start",
            small_caves: BTreeSet::new(),
            can_repeat,
        }
    }

    pub fn position(&self) -> &str {
        self.position
    }

    pub fn step(&self, next: &'a str) -> Option<Self> {
        if self.small_caves.contains(next) {
            if !self.can_repeat {
                None
            } else {
                Some(Self {
                    position: next,
                    small_caves: self.small_caves.clone(),
                    can_repeat: false,
                })
            }
        } else {
            let mut small_caves = self.small_caves.clone();

            if next == next.to_lowercase() {
                small_caves.insert(next);
            }

            Some(Self {
                position: next,
                small_caves,
                can_repeat: self.can_repeat,
            })
        }
    }
}
