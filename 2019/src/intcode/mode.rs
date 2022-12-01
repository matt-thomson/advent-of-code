#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    pub fn from(input: i64) -> Self {
        match input {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("invalid parameter mode {}", input),
        }
    }
}
