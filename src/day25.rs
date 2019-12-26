use std::io::stdin;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Program;
use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day25 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day25 {
    type Output = u32;

    fn part_one(&self) -> u32 {
        let mut computer = Program::read(&self.input).launch();
        println!("{}", output_to_string(&computer.run(&[])));

        while !computer.is_halted() {
            let output = computer.run(&read_input());
            println!("{}", output_to_string(&output));
        }

        0
    }

    fn part_two(&self) -> u32 {
        unimplemented!();
    }
}

fn read_input() -> Vec<i64> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.bytes().map(|x| x as i64).collect()
}

fn output_to_string(output: &[i64]) -> String {
    let bytes: Vec<u8> = output.iter().map(|x| *x as u8).collect();
    String::from_utf8(bytes).unwrap()
}
