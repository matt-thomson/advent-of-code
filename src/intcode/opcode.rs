use super::mode::Mode;

pub enum Opcode {
    Add(Mode, Mode),
    Multiply(Mode, Mode),
    Input,
    Output(Mode),
    Halt,
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode),
    Equals(Mode, Mode),
}

impl Opcode {
    pub fn from(input: i32) -> Self {
        match input % 100 {
            1 => Self::Add(Mode::from(input / 100 % 10), Mode::from(input / 1000 % 10)),
            2 => Self::Multiply(Mode::from(input / 100 % 10), Mode::from(input / 1000 % 10)),
            3 => Self::Input,
            4 => Self::Output(Mode::from(input / 100 % 10)),
            5 => Self::JumpIfTrue(Mode::from(input / 100 % 10), Mode::from(input / 1000 % 10)),
            6 => Self::JumpIfFalse(Mode::from(input / 100 % 10), Mode::from(input / 1000 % 10)),
            7 => Self::LessThan(Mode::from(input / 100 % 10), Mode::from(input / 1000 % 10)),
            8 => Self::Equals(Mode::from(input / 100 % 10), Mode::from(input / 1000 % 10)),
            99 => Self::Halt,
            _ => panic!("invalid opcode {}", input),
        }
    }
}
