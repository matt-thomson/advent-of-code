pub enum Mode {
    Position,
    Immediate,
}

impl Mode {
    pub fn from(input: i32) -> Self {
        match input {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("invalid parameter mode {}", input),
        }
    }
}
