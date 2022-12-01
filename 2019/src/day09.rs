use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Program;
use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day09 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day09 {
    type Output = i64;

    fn part_one(&self) -> i64 {
        self.run(1)
    }

    fn part_two(&self) -> i64 {
        self.run(2)
    }
}

impl Day09 {
    fn run(&self, input: i64) -> i64 {
        let mut computer = Program::read(&self.input).launch();

        let output = computer.run(&[input]);
        assert!(output.len() == 1);

        output[0]
    }
}
