use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Program;
use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day21 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day21 {
    type Output = i64;

    fn part_one(&self) -> i64 {
        let program = Program::read(&self.input);

        let springcode = "NOT A J\n\
                          NOT B T\n\
                          OR T J\n\
                          NOT C T\n\
                          OR T J\n\
                          AND D J\n\
                          WALK\n";

        let input: Vec<i64> = springcode.bytes().map(|x| x as i64).collect();
        let output = program.launch().run(&input);

        output[output.len() - 1]
    }

    fn part_two(&self) -> i64 {
        unimplemented!();
    }
}
