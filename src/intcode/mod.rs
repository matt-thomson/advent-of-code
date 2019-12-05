mod opcode;

use opcode::Opcode;

pub struct Intcode {
    memory: Vec<i32>,
    instruction_pointer: usize,
}

impl Intcode {
    pub fn new(memory: Vec<i32>) -> Self {
        Intcode {
            memory,
            instruction_pointer: 0,
        }
    }

    pub fn run(&mut self, input: &[i32]) -> Vec<i32> {
        let mut input_pointer = 0;
        let mut output = vec![];

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
                Opcode::Input => {
                    let location = self.read() as usize;
                    self.poke(location, input[input_pointer]);
                    input_pointer += 1;
                }
                Opcode::Output => {
                    let location = self.read() as usize;
                    output.push(self.peek(location));
                }
                Opcode::Halt => break,
            }
        }

        output
    }

    pub fn peek(&self, address: usize) -> i32 {
        self.memory[address]
    }

    pub fn poke(&mut self, address: usize, value: i32) {
        self.memory[address] = value;
    }

    fn read(&mut self) -> i32 {
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
        intcode.run(&[]);

        assert_eq!(intcode.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_run_example_two() {
        let mut intcode = Intcode::new(vec![2, 3, 0, 3, 99]);
        intcode.run(&[]);

        assert_eq!(intcode.memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_run_example_three() {
        let mut intcode = Intcode::new(vec![2, 4, 4, 5, 99, 0]);
        intcode.run(&[]);

        assert_eq!(intcode.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_run_example_four() {
        let mut intcode = Intcode::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        intcode.run(&[]);

        assert_eq!(intcode.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_run_example_five() {
        let mut intcode = Intcode::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        intcode.run(&[]);

        assert_eq!(
            intcode.memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_run_input() {
        let mut intcode = Intcode::new(vec![3, 0, 99]);
        intcode.run(&[42]);

        assert_eq!(intcode.memory, vec![42, 0, 99]);
    }

    #[test]
    fn test_run_output() {
        let mut intcode = Intcode::new(vec![4, 0, 99]);
        let output = intcode.run(&[]);

        assert_eq!(output, vec![4]);
    }
}
