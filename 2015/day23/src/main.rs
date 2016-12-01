#[macro_use]
extern crate nom;

mod instruction;
mod machine;
mod program;
mod register;

pub use instruction::Instruction;
pub use machine::Machine;
pub use program::Program;
pub use register::Register;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> u32 {
    solve(filename, 0, 0)
}

fn solve_part_two(filename: &str) -> u32 {
    solve(filename, 1, 0)
}

fn solve(filename: &str, a: u32, b: u32) -> u32 {
    let file = BufReader::new(File::open(filename).unwrap());
    let instructions = file.lines().map(|line| Instruction::parse(&line.unwrap())).collect();
    let program = Program::new(instructions);

    let mut machine = Machine::new(a, b);

    loop {
        let instruction = program.instruction(machine.counter());
        match instruction {
            Some(i) => machine.perform(i),
            None    => break
        }
    }

    machine.b()
}

#[cfg(test)]
mod tests {
    use super::solve_part_one;

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt"), 2);
    }
}
