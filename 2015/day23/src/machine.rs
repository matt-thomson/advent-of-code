use {Instruction, Register};

#[derive(Debug)]
pub struct Machine {
    counter: isize,
    a: u32,
    b: u32
}

impl Machine {
    pub fn new(a: u32, b: u32) -> Machine {
        Machine { counter: 0, a: a, b: b }
    }

    pub fn counter(&self) -> isize {
        self.counter
    }

    pub fn a(&self) -> u32 {
        self.a
    }

    pub fn b(&self) -> u32 {
        self.b
    }

    pub fn perform(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::Half(ref register)                    => self.half(register),
            Instruction::Triple(ref register)                  => self.triple(register),
            Instruction::Increment(ref register)               => self.increment(register),
            Instruction::Jump(offset)                          => self.jump(offset),
            Instruction::JumpIfEven(ref register, offset)      => self.jump_if_even(register, offset),
            Instruction::JumpIfOne(ref register, offset)       => self.jump_if_one(register, offset)
        }
    }

    fn half(&mut self, register: &Register) {
        *self.register_mut(register) /= 2;
        self.counter += 1;
    }

    fn triple(&mut self, register: &Register) {
        *self.register_mut(register) *= 3;
        self.counter += 1;
    }

    fn increment(&mut self, register: &Register) {
        *self.register_mut(register) += 1;
        self.counter += 1;
    }

    fn jump(&mut self, offset: isize) {
        self.counter += offset;
    }

    fn jump_if_even(&mut self, register: &Register, offset: isize) {
        let condition = self.register(register) % 2 == 0;
        self.jump_conditional(offset, condition);
    }

    fn jump_if_one(&mut self, register: &Register, offset: isize) {
        let condition = self.register(register) == 1;
        self.jump_conditional(offset, condition);
    }

    fn jump_conditional(&mut self, offset: isize, condition: bool) {
        self.counter += if condition { offset } else { 1 };
    }

    fn register(&self, register: &Register) -> u32 {
        match *register {
            Register::A => self.a,
            Register::B => self.b
        }
    }

    fn register_mut(&mut self, register: &Register) -> &mut u32 {
        match *register {
            Register::A => &mut self.a,
            Register::B => &mut self.b
        }
    }
}
