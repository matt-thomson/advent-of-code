pub enum Opcode {
    Add,
    Multiply,
    Input,
    Halt,
}

impl Opcode {
    pub fn from(input: u32) -> Opcode {
        match input {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            99 => Self::Halt,
            _ => panic!("invalid opcode {}", input),
        }
    }
}
