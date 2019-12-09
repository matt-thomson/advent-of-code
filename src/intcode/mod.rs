mod instruction;
mod mode;
mod opcode;

use instruction::Instruction;
use mode::Mode;
use opcode::Opcode;

pub struct Intcode {
    memory: Vec<i64>,
    instruction_pointer: usize,
    relative_base: isize,
    is_halted: bool,
}

impl Intcode {
    pub fn new(memory: Vec<i64>) -> Self {
        Intcode {
            memory,
            instruction_pointer: 0,
            relative_base: 0,
            is_halted: false,
        }
    }

    pub fn run(&mut self, input: &[i64]) -> Vec<i64> {
        assert!(!self.is_halted);

        let mut input_pointer = 0;
        let mut output = vec![];

        loop {
            let opcode = Opcode::from(self.read());

            match opcode.instruction() {
                Instruction::Add => {
                    let first = self.read_with_mode(&opcode.mode(0));
                    let second = self.read_with_mode(&opcode.mode(1));
                    let location = self.address(&opcode.mode(2));

                    self.poke(location, first + second);
                }
                Instruction::Multiply => {
                    let first = self.read_with_mode(&opcode.mode(0));
                    let second = self.read_with_mode(&opcode.mode(1));
                    let location = self.address(&opcode.mode(2));

                    self.poke(location, first * second);
                }
                Instruction::Input => {
                    if input_pointer >= input.len() {
                        self.instruction_pointer -= 1;
                        break;
                    }

                    let location = self.address(&opcode.mode(0));

                    self.poke(location, input[input_pointer]);
                    input_pointer += 1;
                }
                Instruction::Output => {
                    let value = self.read_with_mode(&opcode.mode(0));

                    output.push(value);
                }
                Instruction::JumpIfTrue => {
                    let first = self.read_with_mode(&opcode.mode(0));
                    let second = self.read_with_mode(&opcode.mode(1));

                    if first != 0 {
                        self.instruction_pointer = second as usize;
                    }
                }
                Instruction::JumpIfFalse => {
                    let first = self.read_with_mode(&opcode.mode(0));
                    let second = self.read_with_mode(&opcode.mode(1));

                    if first == 0 {
                        self.instruction_pointer = second as usize;
                    }
                }
                Instruction::LessThan => {
                    let first = self.read_with_mode(&opcode.mode(0));
                    let second = self.read_with_mode(&opcode.mode(1));
                    let location = self.address(&opcode.mode(2));

                    let value = if first < second { 1 } else { 0 };
                    self.poke(location, value);
                }
                Instruction::Equals => {
                    let first = self.read_with_mode(&opcode.mode(0));
                    let second = self.read_with_mode(&opcode.mode(1));
                    let location = self.address(&opcode.mode(2));

                    let value = if first == second { 1 } else { 0 };
                    self.poke(location, value);
                }
                Instruction::SetRelativeBase => {
                    self.relative_base += self.read_with_mode(&opcode.mode(0)) as isize;
                }
                Instruction::Halt => {
                    self.is_halted = true;
                    break;
                }
            }
        }

        output
    }

    pub fn peek(&mut self, address: usize) -> i64 {
        self.resize(address);
        self.memory[address]
    }

    pub fn poke(&mut self, address: usize, value: i64) {
        self.resize(address);
        self.memory[address] = value;
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    fn read(&mut self) -> i64 {
        let value = self.peek(self.instruction_pointer);
        self.instruction_pointer += 1;

        value
    }

    fn read_with_mode(&mut self, mode: &Mode) -> i64 {
        let value = self.read();
        match mode {
            Mode::Position => self.peek(value as usize),
            Mode::Immediate => value,
            Mode::Relative => {
                let location = self.relative_base + (value as isize);
                self.peek(location as usize)
            }
        }
    }

    fn address(&mut self, mode: &Mode) -> usize {
        let value = self.read() as usize;
        match mode {
            Mode::Position => value as usize,
            Mode::Immediate => panic!("can't get address in immediate mode"),
            Mode::Relative => (self.relative_base + value as isize) as usize,
        }
    }

    fn resize(&mut self, address: usize) {
        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
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

    #[test]
    fn test_run_relative_mode() {
        let code = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut intcode = Intcode::new(code.clone());
        let output = intcode.run(&[]);

        assert_eq!(output, code);
    }

    #[test]
    fn test_large_multiplication() {
        let mut intcode = Intcode::new(vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0]);
        let output = intcode.run(&[]);

        assert_eq!(output, vec![1_219_070_632_396_864]);
    }

    #[test]
    fn test_large_number() {
        let mut intcode = Intcode::new(vec![104, 1_125_899_906_842_624, 99]);
        let output = intcode.run(&[]);

        assert_eq!(output, vec![1_125_899_906_842_624]);
    }

    #[test]
    fn test_run_relative_write() {
        let mut intcode = Intcode::new(vec![109, 100, 21101, 2, 3, 200, 4, 300, 99]);
        let output = intcode.run(&[]);

        assert_eq!(output, vec![5]);
    }
}
