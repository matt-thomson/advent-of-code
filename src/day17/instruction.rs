#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

impl Instruction {
    pub fn to_string(&self) -> String {
        match self {
            Self::Move(steps) => steps.to_string(),
            Self::TurnLeft => "L".to_string(),
            Self::TurnRight => "R".to_string(),
        }
    }
}
