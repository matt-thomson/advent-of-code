use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Program;
use crate::problem::Problem;

#[derive(Debug, StructOpt)]
pub struct Day19 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day19 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let program = Program::read(&self.input);

        (0..50)
            .flat_map(|x| (0..50).map(move |y| (x, y)))
            .filter(|(x, y)| in_beam(&program, *x, *y))
            .count()
    }

    fn part_two(&self) -> usize {
        unimplemented!();
    }
}

fn in_beam(program: &Program, x: usize, y: usize) -> bool {
    let output = program.launch().run(&[x as i64, y as i64]);
    assert_eq!(output.len(), 1);

    output[0] == 1
}
