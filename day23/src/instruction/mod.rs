mod parse;

use Register;

#[derive(Debug)]
pub enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize)
}

impl Instruction {
    pub fn parse(line: &str) -> Instruction {
        parse::parse(line)
    }
}

#[cfg(test)]
mod tests {
    use Instruction;

    #[test]
    fn test_parse() {
        let instruction = Instruction::parse("jmp +23");
        match instruction {
            Instruction::Jump(23) => (),
            _                     => panic!("Incorrect instruction {:?}", instruction)
        }
    }
}
