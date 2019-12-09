pub enum Instruction {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    SetRelativeBase,
    Halt,
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
            9 => Instruction::SetRelativeBase,
            99 => Instruction::Halt,
            _ => panic!("invalid opcode {}", value % 100),
        }
    }
}
