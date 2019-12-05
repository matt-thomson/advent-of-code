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
    Add {
        first_mode: ParameterMode,
        second_mode: ParameterMode,
    },
    Multiply {
        first_mode: ParameterMode,
        second_mode: ParameterMode,
    },
    Input,
    Output {
        mode: ParameterMode,
    },
    Halt,
}

impl Opcode {
    pub fn from(input: i32) -> Self {
        match input % 100 {
            1 => Self::Add {
                first_mode: ParameterMode::from(input / 100 % 10),
                second_mode: ParameterMode::from(input / 1000 % 10),
            },
            2 => Self::Multiply {
                first_mode: ParameterMode::from(input / 100 % 10),
                second_mode: ParameterMode::from(input / 1000 % 10),
            },
            3 => Self::Input,
            4 => Self::Output {
                mode: ParameterMode::from(input / 100 % 10),
            },
            99 => Self::Halt,
            _ => panic!("invalid opcode {}", input),
        }
    }
}
