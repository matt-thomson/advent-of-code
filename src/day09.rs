use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Intcode;
use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day09 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day09 {
    type Output = i64;

    fn part_one(&self) -> i64 {
        let program: Vec<i64> = fs::read_to_string(&self.input)
            .unwrap()
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let mut intcode = Intcode::new(program);

        let output = intcode.run(&[1]);
        assert!(output.len() == 1);

        output[0]
    }

    fn part_two(&self) -> i64 {
        unimplemented!()
    }
}
