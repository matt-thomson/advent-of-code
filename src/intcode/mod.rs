mod mode;
mod opcode;

use mode::Mode;
use opcode::Opcode;

pub struct Intcode {
    memory: Vec<i32>,
    instruction_pointer: usize,
    is_halted: bool,
}

impl Intcode {
    pub fn new(memory: Vec<i32>) -> Self {
        Intcode {
            memory,
            instruction_pointer: 0,
            is_halted: false,
        }
    }

    pub fn run(&mut self, input: &[i32]) -> Vec<i32> {
        assert!(!self.is_halted);

        let mut input_pointer = 0;
        let mut output = vec![];

        loop {
            match Opcode::from(self.read()) {
                Opcode::Add(first_mode, second_mode) => {
                    let first = self.read_with_mode(&first_mode);
                    let second = self.read_with_mode(&second_mode);
                    let location = self.read() as usize;
                    self.poke(location, first + second);
                }
                Opcode::Multiply(first_mode, second_mode) => {
                    let first = self.read_with_mode(&first_mode);
                    let second = self.read_with_mode(&second_mode);
                    let location = self.read() as usize;
                    self.poke(location, first * second);
                }
                Opcode::Input => {
                    if input_pointer >= input.len() {
                        self.instruction_pointer -= 1;
                        break;
                    }

                    let location = self.read() as usize;
                    self.poke(location, input[input_pointer]);
                    input_pointer += 1;
                }
                Opcode::Output(mode) => {
                    let value = self.read_with_mode(&mode);
                    output.push(value);
                }
                Opcode::JumpIfTrue(first_mode, second_mode) => {
                    let first = self.read_with_mode(&first_mode);
                    let second = self.read_with_mode(&second_mode);

                    if first != 0 {
                        self.instruction_pointer = second as usize;
                    }
                }
                Opcode::JumpIfFalse(first_mode, second_mode) => {
                    let first = self.read_with_mode(&first_mode);
                    let second = self.read_with_mode(&second_mode);

                    if first == 0 {
                        self.instruction_pointer = second as usize;
                    }
                }
                Opcode::LessThan(first_mode, second_mode) => {
                    let first = self.read_with_mode(&first_mode);
                    let second = self.read_with_mode(&second_mode);
                    let location = self.read() as usize;

                    let value = if first < second { 1 } else { 0 };
                    self.poke(location, value);
                }
                Opcode::Equals(first_mode, second_mode) => {
                    let first = self.read_with_mode(&first_mode);
                    let second = self.read_with_mode(&second_mode);
                    let location = self.read() as usize;

                    let value = if first == second { 1 } else { 0 };
                    self.poke(location, value);
                }
                Opcode::Halt => {
                    self.is_halted = true;
                    break;
                }
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

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    fn read(&mut self) -> i32 {
        let value = self.memory[self.instruction_pointer];
        self.instruction_pointer += 1;

        value
    }

    fn read_with_mode(&mut self, mode: &Mode) -> i32 {
        let value = self.read();
        match mode {
            Mode::Position => self.peek(value as usize),
            Mode::Immediate => value,
        }
    }
}

mod tests {
    use super::Intcode;

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

    #[test]
    fn test_run_immediate_mode() {
        let mut intcode = Intcode::new(vec![1101, 100, -1, 4, 0]);
        intcode.run(&[]);

        assert_eq!(intcode.memory, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_run_less_than_true() {
        let mut intcode = Intcode::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        let output = intcode.run(&[7]);

        assert_eq!(output, vec![1]);
    }

    #[test]
    fn test_run_less_than_false() {
        let mut intcode = Intcode::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        let output = intcode.run(&[9]);

        assert_eq!(output, vec![0]);
    }

    #[test]
    fn test_run_equals_true() {
        let mut intcode = Intcode::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let output = intcode.run(&[8]);

        assert_eq!(output, vec![1]);
    }

    #[test]
    fn test_run_equals_false() {
        let mut intcode = Intcode::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let output = intcode.run(&[9]);

        assert_eq!(output, vec![0]);
    }
}
