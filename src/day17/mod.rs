mod direction;
mod image;
mod instruction;
mod pixel;

use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Program;
use crate::problem::Problem;

use image::Image;

#[derive(Debug, StructOpt)]
pub struct Day17 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day17 {
    type Output = usize;

    fn part_one(&self) -> usize {
        let mut computer = Program::read(&self.input).launch();
        let output = computer.run(&[]);
        let image = Image::new(&output);

        image.intersections().iter().map(|(x, y)| x * y).sum()
    }

    fn part_two(&self) -> usize {
        let mut computer = Program::read(&self.input).launch();
        computer.poke(0, 2);

        let output = computer.run(&[]);
        let image = Image::new(&output);

        dbg!(image.path());

        unimplemented!();
    }
}
