use Instruction;

pub struct Program {
    instructions: Vec<Instruction>
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Program {
        Program { instructions: instructions }
    }

    pub fn instruction(&self, index: isize) -> Option<&Instruction> {
        if index < 0 { None } else { self.instructions.get(index as usize) }
    }
}

#[cfg(test)]
mod tests {
    use {Instruction, Program, Register};

    fn program() -> Program {
        let instructions = vec![
            Instruction::Half(Register::A),
            Instruction::Increment(Register::B),
            Instruction::Jump(23)
        ];

        Program::new(instructions)
    }

    #[test]
    fn test_instruction_within_range() {
        let program = program();
        let instruction = program.instruction(1);

        match instruction {
            Some(&Instruction::Increment(Register::B)) => (),
            _                                          => panic!("Incorrect instruction {:?}", instruction)
        }
    }

    #[test]
    fn test_instruction_after_range() {
        let program = program();
        let instruction = program.instruction(3);

        match instruction {
            None => (),
            _    => panic!("Incorrect instruction {:?}", instruction)
        }
    }

    #[test]
    fn test_instruction_negative() {
        let program = program();
        let instruction = program.instruction(-1);

        match instruction {
            None => (),
            _    => panic!("Incorrect instruction {:?}", instruction)
        }
    }
}
