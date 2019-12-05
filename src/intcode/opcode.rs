pub enum Opcode {
    Add,
    Multiply,
    Halt,
}

impl Opcode {
    pub fn from(input: u32) -> Opcode {
        match input {
            1 => Self::Add,
            2 => Self::Multiply,
            99 => Self::Halt,
            _ => panic!("invalid opcode {}", input),
        }
    }
}
