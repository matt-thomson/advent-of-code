#[derive(Debug)]
pub struct Player {
    position: u32,
    pub score: u32,
}

impl Player {
    pub fn new(position: u32) -> Self {
        Self { position, score: 0 }
    }

    pub fn step(&mut self, roll: u32) {
        self.position = (self.position + roll - 1) % 10 + 1;
        self.score += self.position;
    }
}
