pub struct Dice {
    numbers: Box<dyn Iterator<Item = u32>>,
    pub rolls: u32,
}

impl Dice {
    pub fn new() -> Self {
        let numbers = Box::new((1..=1000).cycle());
        Self { numbers, rolls: 0 }
    }

    pub fn roll(&mut self) -> u32 {
        self.rolls += 3;
        self.numbers.by_ref().take(3).sum()
    }
}
