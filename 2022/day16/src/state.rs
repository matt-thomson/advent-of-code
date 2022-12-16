#[derive(Debug, Hash, PartialEq, Eq)]
pub struct State {
    position: String,
    opened: Vec<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            position: "AA".to_string(),
            opened: Vec::default(),
        }
    }
}

impl State {
    pub fn position(&self) -> &str {
        &self.position
    }

    pub fn step(&self, position: &str) -> Self {
        Self {
            position: position.to_string(),
            opened: self.opened.clone(),
        }
    }

    pub fn open(&self, valve: &str) -> Self {
        let mut opened = self.opened.clone();
        opened.push(valve.to_string());
        opened.sort_unstable();

        Self {
            position: self.position.clone(),
            opened,
        }
    }

    pub fn is_open(&self, name: &str) -> bool {
        self.opened.contains(&name.to_string())
    }
}
