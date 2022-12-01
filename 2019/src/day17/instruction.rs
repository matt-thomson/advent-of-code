use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Move(steps) => write!(f, "{}", steps),
            Self::TurnLeft => write!(f, "L"),
            Self::TurnRight => write!(f, "R"),
        }
    }
}
