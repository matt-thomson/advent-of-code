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
    fn part_one(&self) -> u32 {
        let program = self.read_program();
        let mut intcode = Intcode::new(program);

        let output = intcode.run(&[1]);

        let tests_passed = output[0..output.len() - 1].iter().all(|x| *x == 0);
        assert!(tests_passed);

        output[output.len() - 1] as u32
    }

    fn part_two(&self) -> u32 {
        unimplemented!()
    }
}

impl Day05 {
    fn read_program(&self) -> Vec<i32> {
        fs::read_to_string(&self.input)
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect()
    }
}
