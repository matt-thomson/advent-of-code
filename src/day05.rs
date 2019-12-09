use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::command;
use crate::intcode::Intcode;

#[derive(Debug, StructOpt)]
pub struct Day05 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl command::Command for Day05 {
    type Output = i32;

    fn part_one(&self) -> i32 {
        let output = self.run(1);

        let tests_passed = output[0..output.len() - 1].iter().all(|x| *x == 0);
        assert!(tests_passed);

        output[output.len() - 1]
    }

    fn part_two(&self) -> i32 {
        let output = self.run(5);
        assert!(output.len() == 1);

        output[0]
    }
}

impl Day05 {
    fn run(&self, input: i32) -> Vec<i32> {
        let program: Vec<i32> = fs::read_to_string(&self.input)
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let mut intcode = Intcode::new(program);

        intcode.run(&[input])
    }
}
