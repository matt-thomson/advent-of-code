use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Route {
    pub position: String,
    pub small_caves: BTreeSet<String>,
}

impl Default for Route {
    fn default() -> Self {
        Self {
            position: "start".to_string(),
            small_caves: BTreeSet::new(),
        }
    }
}

impl Route {
    pub fn step(&self, next: &str) -> Option<Self> {
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
