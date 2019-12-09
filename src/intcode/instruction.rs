pub enum Instruction {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

impl Instruction {
    pub fn from(value: i32) -> Instruction {
        match value {
            1 => Instruction::Add,
            2 => Instruction::Multiply,
            3 => Instruction::Input,
            4 => Instruction::Output,
            5 => Instruction::JumpIfTrue,
            6 => Instruction::JumpIfFalse,
            7 => Instruction::LessThan,
            8 => Instruction::Equals,
            99 => Instruction::Halt,
            _ => panic!("invalid opcode {}", value % 100),
        }
    }
}
