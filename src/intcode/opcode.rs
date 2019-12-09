use super::instruction::Instruction;
use super::mode::Mode;

pub struct Opcode(i64);

impl Opcode {
    pub fn from(value: i64) -> Opcode {
        Opcode(value)
    }

    pub fn instruction(&self) -> Instruction {
        Instruction::from(self.0 % 100)
    }

    pub fn mode(&self, index: u32) -> Mode {
        Mode::from(self.0 / 10_i64.pow(index + 2) % 10)
    }
}
