use std::collections::VecDeque;

use crate::instruction::Instruction;

pub struct Cpu {
    x: i64,
    instructions: VecDeque<Instruction>,
    pending_add: Option<i64>,
}

impl Cpu {
    pub fn new(instructions: &[Instruction]) -> Self {
        Self {
            x: 1,
            instructions: instructions.iter().copied().collect(),
            pending_add: None,
        }
    }
}

impl Iterator for Cpu {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.pending_add {
            self.x += value;
            self.pending_add = None;
        } else if let Some(instruction) = self.instructions.pop_front() {
            if let Instruction::Add(value) = instruction {
                self.pending_add = Some(value);
            }
        } else {
            return None;
        }

        Some(self.x)
    }
}
