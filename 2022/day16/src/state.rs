#[derive(Default, Debug, Hash, PartialEq, Eq)]
pub struct State {
    position: u64,
    opened: Vec<u64>,
}

impl State {
    pub fn position(&self) -> u64 {
        self.position
    }

    pub fn step(&self, position: u64) -> Self {
        Self {
            position,
            opened: self.opened.clone(),
        }
    }

    pub fn open(&self, valve: u64) -> Self {
        let mut opened = self.opened.clone();
        opened.push(valve);
        opened.sort_unstable();

        Self {
            position: self.position,
            opened,
        }
    }

    pub fn is_open(&self, name: u64) -> bool {
        self.opened.contains(&name)
    }
}
