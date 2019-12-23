pub struct Packet {
    address: usize,
    x: i64,
    y: i64,
}

impl Packet {
    pub fn from(input: &[i64]) -> Self {
        Packet {
            address: input[0] as usize,
            x: input[1],
            y: input[2],
        }
    }

    pub fn address(&self) -> usize {
        self.address
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }
}
