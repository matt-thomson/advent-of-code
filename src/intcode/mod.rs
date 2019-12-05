mod opcode;

use opcode::Opcode;

pub struct Intcode {
    memory: Vec<u32>,
    instruction_pointer: usize,
}

impl Intcode {
    pub fn new(memory: Vec<u32>) -> Self {
        Intcode {
            memory,
            instruction_pointer: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            match Opcode::from(self.read()) {
                Opcode::Add => {
                    let first = self.read() as usize;
                    let second = self.read() as usize;
                    let location = self.read() as usize;
                    self.poke(location, self.peek(first) + self.peek(second));
                }
                Opcode::Multiply => {
                    let first = self.read() as usize;
                    let second = self.read() as usize;
                    let location = self.read() as usize;
                    self.poke(location, self.peek(first) * self.peek(second));
                }
                Opcode::Halt => break,
            }
        }
    }

    pub fn peek(&self, address: usize) -> u32 {
        self.memory[address]
    }

    pub fn poke(&mut self, address: usize, value: u32) {
        self.memory[address] = value;
    }

    fn read(&mut self) -> u32 {
        let value = self.memory[self.instruction_pointer];
        self.instruction_pointer += 1;

        value
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_run_example_one() {
        let mut intcode = Intcode::new(vec![1, 0, 0, 0, 99]);
        intcode.run();

        assert_eq!(intcode.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_run_example_two() {
        let mut intcode = Intcode::new(vec![2, 3, 0, 3, 99]);
        intcode.run();

        assert_eq!(intcode.memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_run_example_three() {
        let mut intcode = Intcode::new(vec![2, 4, 4, 5, 99, 0]);
        intcode.run();

        assert_eq!(intcode.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_run_example_four() {
        let mut intcode = Intcode::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        intcode.run();

        assert_eq!(intcode.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_run_example_five() {
        let mut intcode = Intcode::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        intcode.run();

        assert_eq!(
            intcode.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
}
