pub enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    pub fn from(input: i32) -> Self {
        match input {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("invalid parameter mode {}", input),
        }
    }
}

pub enum Opcode {
    Add(ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    Halt,
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
}

impl Opcode {
    pub fn from(input: i32) -> Self {
        match input % 100 {
            1 => Self::Add(
                ParameterMode::from(input / 100 % 10),
                ParameterMode::from(input / 1000 % 10),
            ),
            2 => Self::Multiply(
                ParameterMode::from(input / 100 % 10),
                ParameterMode::from(input / 1000 % 10),
            ),
            3 => Self::Input,
            4 => Self::Output(ParameterMode::from(input / 100 % 10)),
            5 => Self::JumpIfTrue(
                ParameterMode::from(input / 100 % 10),
                ParameterMode::from(input / 1000 % 10),
            ),
            6 => Self::JumpIfFalse(
                ParameterMode::from(input / 100 % 10),
                ParameterMode::from(input / 1000 % 10),
            ),
            7 => Self::LessThan(
                ParameterMode::from(input / 100 % 10),
                ParameterMode::from(input / 1000 % 10),
            ),
            8 => Self::Equals(
                ParameterMode::from(input / 100 % 10),
                ParameterMode::from(input / 1000 % 10),
            ),
            99 => Self::Halt,
            _ => panic!("invalid opcode {}", input),
        }
    }
}
