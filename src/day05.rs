use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Program;
use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day05 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day05 {
    type Output = i64;

    fn part_one(&self) -> i64 {
        let output = self.run(1);

        let tests_passed = output[0..output.len() - 1].iter().all(|x| *x == 0);
        assert!(tests_passed);

        output[output.len() - 1]
    }

    fn part_two(&self) -> i64 {
        let output = self.run(5);
        assert!(output.len() == 1);

        output[0]
    }
}

impl Day05 {
    fn run(&self, input: i64) -> Vec<i64> {
        let mut computer = Program::read(&self.input).launch();
        computer.run(&[input])
    }
}
